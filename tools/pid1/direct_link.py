"""Bind TV diagnostics to the routed local link, including Internet Sharing."""
import re
import socket
import subprocess

HOST = '192.168.77.1'
TV = '192.168.77.2'


def interface_from(route, config):
    match = re.search(r'^\s*interface:\s*(\S+)\s*$', route, re.MULTILINE)
    if not match or not re.search(r'^\s*inet 192\.168\.77\.1 netmask 0xffffff00(?:\s|$)', config, re.MULTILINE):
        raise OSError('The TV debug route must have local address 192.168.77.1/24; refusing the default Wi-Fi route')
    return match.group(1)


def interface():
    route = subprocess.check_output(['route', '-n', 'get', TV], text=True)
    match = re.search(r'^\s*interface:\s*(\S+)\s*$', route, re.MULTILINE)
    if not match:
        raise OSError('No interface for the TV debug route')
    config = subprocess.check_output(['ifconfig', match.group(1)], text=True)
    return interface_from(route, config)


def connect(port, timeout):
    name = interface()
    connection = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    try:
        connection.setsockopt(socket.IPPROTO_IP, getattr(socket, 'IP_BOUND_IF', 25), socket.if_nametoindex(name))
        connection.bind((HOST, 0))
        connection.settimeout(timeout)
        connection.connect((TV, port))
        return connection
    except BaseException:
        connection.close()
        raise
