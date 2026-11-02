/* Native private-LAN transport for Chocolate Doom 2.2.1's original protocol.
 * No SDL, resolver, Internet master, network GUI or additional thread.
 * Numeric IPv4 and the standard UDP port 2342 only. GPL-2.0-or-later. */
#include <arpa/inet.h>
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>
#include "doomtype.h"
#include "i_system.h"
#include "i_timer.h"
#include "net_defs.h"
#include "net_io.h"
#include "net_packet.h"
#include "net_sdl.h"
#include "net_client.h"
#include "net_server.h"
#include "net_query.h"

#define PORT 2342
#define ADDRESSES 64
#define PACKET_MAX 1500
static int udp = -1;
typedef struct { net_addr_t net; struct sockaddr_in ip; boolean used; } address_t;
static address_t addresses[ADDRESSES];

static net_addr_t *intern(struct sockaddr_in *ip)
{
    int i, empty = -1;
    for (i = 0; i < ADDRESSES; ++i) {
        if (!addresses[i].used) { if (empty < 0) empty = i; continue; }
        if (addresses[i].ip.sin_addr.s_addr == ip->sin_addr.s_addr &&
            addresses[i].ip.sin_port == ip->sin_port) return &addresses[i].net;
    }
    if (empty < 0) return NULL; /* Bounded state even for unsolicited UDP peers. */
    address_t *a = &addresses[empty];
    a->used = true; a->ip = *ip;
    a->net.module = &net_sdl_module; a->net.handle = &a->ip;
    return &a->net;
}
static boolean init_socket(boolean server)
{
    if (udp >= 0) return true;
    udp = socket(AF_INET, SOCK_DGRAM | SOCK_CLOEXEC, 0);
    if (udp < 0) I_Error("doom net: socket: %s", strerror(errno));
    struct sockaddr_in bind_to = {0};
    bind_to.sin_family = AF_INET;
    bind_to.sin_port = htons(server ? PORT : 0);
    bind_to.sin_addr.s_addr = htonl(INADDR_ANY);
    if (bind(udp, (struct sockaddr *)&bind_to, sizeof(bind_to)) < 0)
        I_Error("doom net: bind UDP %d: %s", server ? PORT : 0, strerror(errno));
    if (fcntl(udp, F_SETFL, O_NONBLOCK) < 0) I_Error("doom net: nonblocking socket");
    printf("doom net: native UDP %s ready\n", server ? "host" : "client");
    return true;
}
static boolean init_client(void) { return init_socket(false); }
static boolean init_server(void) { return init_socket(true); }
static void send_packet(net_addr_t *addr, net_packet_t *packet)
{
    /* Discovery broadcasts deliberately unsupported; explicit peer only. */
    if (addr == &net_broadcast_addr || packet->len > PACKET_MAX) return;
    struct sockaddr_in *to = addr->handle;
    if (sendto(udp, packet->data, packet->len, 0, (struct sockaddr *)to, sizeof(*to)) < 0 &&
        errno != EAGAIN && errno != ENOBUFS && errno != EINTR)
        I_Error("doom net: send: %s", strerror(errno));
}
static boolean receive_packet(net_addr_t **addr, net_packet_t **packet)
{
    byte data[PACKET_MAX];
    struct sockaddr_in from; socklen_t len = sizeof(from);
    ssize_t n = recvfrom(udp, data, sizeof(data), MSG_TRUNC, (struct sockaddr *)&from, &len);
    if (n < 0) {
        if (errno == EAGAIN || errno == EWOULDBLOCK || errno == EINTR) return false;
        I_Error("doom net: receive: %s", strerror(errno));
    }
    if (!n || n > sizeof(data) || len != sizeof(from) || from.sin_family != AF_INET) return false;
    *addr = intern(&from);
    if (!*addr) return false;
    *packet = NET_NewPacket(n);
    memcpy((*packet)->data, data, n); (*packet)->len = n;
    return true;
}
static void address_string(net_addr_t *addr, char *out, int size)
{
    struct sockaddr_in *ip = addr->handle;
    char text[INET_ADDRSTRLEN];
    inet_ntop(AF_INET, &ip->sin_addr, text, sizeof(text));
    snprintf(out, size, "%s:%u", text, ntohs(ip->sin_port));
}
static void free_address(net_addr_t *addr)
{
    int i; for (i = 0; i < ADDRESSES; ++i)
        if (&addresses[i].net == addr) { addresses[i].used = false; return; }
    I_Error("doom net: invalid address");
}
static net_addr_t *resolve_address(char *text)
{
    struct sockaddr_in ip = {0};
    ip.sin_family = AF_INET; ip.sin_port = htons(PORT);
    if (!text || inet_pton(AF_INET, text, &ip.sin_addr) != 1) return NULL;
    return intern(&ip);
}
/* Keep the original protocol's module name; implementation has no SDL. */
net_module_t net_sdl_module = {
    init_client, init_server, send_packet, receive_packet,
    address_string, free_address, resolve_address
};

void NET_WaitForLaunch(void)
{
    int begin = I_GetTimeMS(); boolean launched = false;
    printf("doom net: waiting for two players (30 second deadline)\n");
    while (net_waiting_for_launch) {
        NET_CL_Run(); NET_SV_Run();
        if (!net_client_connected) I_Error("doom net: disconnected before launch");
        if (net_client_received_wait_data) {
            if (memcmp(net_local_wad_sha1sum, net_client_wait_data.wad_sha1sum, 20))
                I_Error("doom net: IWAD directory mismatch");
            if (!launched && net_client_wait_data.is_controller && net_client_wait_data.num_players >= 2) {
                printf("doom net: launching %d-player game\n", net_client_wait_data.num_players);
                NET_CL_LaunchGame(); launched = true;
            }
        }
        if (I_GetTimeMS() - begin > 30000) I_Error("doom net: second player timeout");
        I_Sleep(10);
    }
}
/* The TV supports explicit private LAN peers only. Never resolve/contact an
 * Internet master or silently enable discovery/dedicated-server behavior. */
net_addr_t *NET_Query_ResolveMaster(net_context_t *c) { (void)c; return NULL; }
void NET_Query_AddToMaster(net_addr_t *a) { (void)a; }
void NET_Query_MasterResponse(net_packet_t *p) { (void)p; }
void NET_MasterQuery(void) { I_Error("doom net: Internet discovery disabled"); }
void NET_LANQuery(void) { I_Error("doom net: use an explicit IPv4 peer"); }
void NET_QueryAddress(char *a) { (void)a; I_Error("doom net: queries disabled"); }
net_addr_t *NET_FindLANServer(void) { return NULL; }
void NET_DedicatedServer(void) { I_Error("doom net: dedicated mode disabled"); }

/* Once per game second, expose synchronized gameplay state, not just traffic.
 * Test peers compare this hash at identical tics and inject distinct inputs. */
#include "doomstat.h"
void doomtv_net_trace(void)
{
    unsigned int hash = 2166136261u;
    int i, j;
    for (i = 0; i < MAXPLAYERS; ++i) {
        player_t *p = &players[i];
        unsigned int values[] = {playeringame[i], p->health,
            p->mo ? p->mo->x : 0, p->mo ? p->mo->y : 0,
            p->mo ? p->mo->angle : 0, p->ammo[0],
            p->cmd.forwardmove, p->cmd.sidemove, p->cmd.buttons};
        for (j = 0; j < sizeof(values) / sizeof(values[0]); ++j)
            hash = (hash ^ values[j]) * 16777619u;
    }
    printf("NETSTATE tic=%d local=%d hash=%08x p0=%d,%d p1=%d,%d cmd0=%d,%d cmd1=%d,%d\n",
        gametic, consoleplayer, hash,
        players[0].mo ? players[0].mo->x : 0, players[0].mo ? players[0].mo->y : 0,
        players[1].mo ? players[1].mo->x : 0, players[1].mo ? players[1].mo->y : 0,
        players[0].cmd.forwardmove, players[0].cmd.buttons,
        players[1].cmd.forwardmove, players[1].cmd.buttons);
    fflush(stdout);
}
