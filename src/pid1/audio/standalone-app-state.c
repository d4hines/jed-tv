/* Standalone audioserver only: there is no ActivityManager/system_server.
 * Skip UID observer registration and report the directly launched clients as
 * active. ABI names are from the exact V109 libaudiopolicyservice imports.
 * Load only into audioserver, never into a normal Android boot.
 * G13/AU13 has the same imports and 48-byte UidInfo result. When linked as a
 * DT_NEEDED dependency, build with -z global: dlopened libaudiohal@7.1 also
 * calls requestPriority and otherwise waits forever for scheduling_policy.
 */
int register_uid(void) __asm__("_ZN7android15ActivityManager19registerUidObserverERKNS_2spINS_12IUidObserverEEEiiRKNS_8String16E");
int register_uid(void) { return 0; }
int unregister_uid(void) __asm__("_ZN7android15ActivityManager21unregisterUidObserverERKNS_2spINS_12IUidObserverEEE");
int unregister_uid(void) { return 0; }
int link_death(void) __asm__("_ZN7android15ActivityManager11linkToDeathERKNS_2spINS_7IBinder14DeathRecipientEEE");
int link_death(void) { return 0; }
int unlink_death(void) __asm__("_ZN7android15ActivityManager13unlinkToDeathERKNS_2spINS_7IBinder14DeathRecipientEEE");
int unlink_death(void) { return 0; }
int uid_active(void) __asm__("_ZN7android15ActivityManager11isUidActiveEjRKNS_8String16E");
int uid_active(void) { return 1; }
int uid_state(void) __asm__("_ZN7android15ActivityManager18getUidProcessStateEjRKNS_8String16E");
int uid_state(void) { return 2; }
/* No microphone use in this standalone player: leave sensor privacy enabled. */
int privacy_enabled(void) __asm__("_ZN7android20SensorPrivacyManager22isSensorPrivacyEnabledEv");
int privacy_enabled(void) { return 1; }
int add_privacy_listener(void) __asm__("_ZN7android20SensorPrivacyManager24addSensorPrivacyListenerERKNS_2spINS_8hardware22ISensorPrivacyListenerEEE");
int add_privacy_listener(void) { return 0; }
int remove_privacy_listener(void) __asm__("_ZN7android20SensorPrivacyManager27removeSensorPrivacyListenerERKNS_2spINS_8hardware22ISensorPrivacyListenerEEE");
int remove_privacy_listener(void) { return 0; }
/* Native uid-0 clients have no Android application package to enumerate. */
int packages_for_uid(void) __asm__("_ZN7android20PermissionController17getPackagesForUidEjRNS_6VectorINS_8String16EEE");
int packages_for_uid(void) { return 0; }
/* V109 ARM32 UidInfo::Info is 48 bytes: uid, two libc++ small strings,
 * and numeric metadata. Preserve empty strings; native clients have no APK.
 * ARM AAPCS returns this aggregate through the first (hidden) argument.
 */
void uid_info(void *out, void *self, unsigned uid) __asm__("_ZN7android10mediautils7UidInfo7getInfoEj");
void uid_info(void *out, void *self, unsigned uid) {
    (void)self;
    unsigned char *bytes = out;
    for (unsigned i=0; i<48; ++i) bytes[i]=0;
    *(unsigned *)out=uid;
}
/* There is no framework scheduling daemon. Fail optional priority promotion
 * immediately instead of sleeping forever; ordinary Linux scheduling remains. */
int request_priority(void) __asm__("_ZN7android15requestPriorityEiiibb");
int request_priority(void) { return -38; }
