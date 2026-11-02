import unittest
from direct_link import interface_from

class RouteTests(unittest.TestCase):
    def test_direct_adapter(self):
        self.assertEqual(interface_from(' interface: en8\n', 'en8: flags=1\n inet 192.168.77.1 netmask 0xffffff00 broadcast 192.168.77.255\n'), 'en8')

    def test_sharing_bridge_with_both_addresses(self):
        self.assertEqual(interface_from(' interface: bridge100\n', 'bridge100: flags=1\n inet 192.168.2.1 netmask 0xffffff00\n inet 192.168.77.1 netmask 0xffffff00\n'), 'bridge100')

    def test_default_wifi_or_wrong_subnet_is_refused(self):
        for config in [' inet 10.104.0.212 netmask 0xffff0000\n', ' inet 192.168.77.1 netmask 0xffff0000\n', ' inet 192.168.77.10 netmask 0xffffff00\n']:
            with self.assertRaises(OSError):
                interface_from(' interface: en0\n', config)

if __name__ == '__main__': unittest.main()
