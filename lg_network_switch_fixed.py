#!/usr/bin/env python3
"""
LG TV Network Input Switcher - Fixed version with proper key storage
"""

import sys
import asyncio
import os
import json
from typing import Optional

try:
    from bscpylgtv import WebOsClient
except ImportError:
    print("ERROR: bscpylgtv not installed!")
    print("\nInstall it with:")
    print("  pip3 install bscpylgtv")
    sys.exit(1)


class SimpleKeyStore:
    """Simple file-based key storage for bscpylgtv"""

    def __init__(self, storage_path=None):
        if storage_path is None:
            home = os.path.expanduser("~")
            storage_path = os.path.join(home, ".lg_tv_keys.json")
        self.storage_path = storage_path
        self.keys = self._load()

    def _load(self):
        """Load keys from file"""
        try:
            with open(self.storage_path, 'r') as f:
                return json.load(f)
        except (FileNotFoundError, json.JSONDecodeError):
            return {}

    def _save(self):
        """Save keys to file"""
        try:
            with open(self.storage_path, 'w') as f:
                json.dump(self.keys, f)
        except Exception as e:
            print(f"Warning: Could not save key: {e}")

    async def get_key(self, tv_ip):
        """Get stored key for TV"""
        return self.keys.get(tv_ip)

    async def set_key(self, tv_ip, key):
        """Store key for TV"""
        self.keys[tv_ip] = key
        self._save()


async def switch_input(tv_ip: str, input_name: str):
    """Switch LG TV input via network API"""

    # Create key storage
    store = SimpleKeyStore()

    # Get stored key
    client_key = await store.get_key(tv_ip)
    if client_key:
        print(f"✓ Using saved credentials")
    else:
        print(f"⚠ First time connection - pairing required")
        print(f"  Accept the prompt on your TV!")

    # Create client with storage
    client = WebOsClient(tv_ip, client_key=client_key, storage=store)

    try:
        print(f"Connecting to {tv_ip}...")
        await client.connect()
        print(f"✓ Connected!")

        # Get current input
        try:
            current = await client.get_input()
            print(f"Current input: {current}")
        except Exception as e:
            print(f"Could not read current input: {e}")

        # Switch input
        print(f"Switching to {input_name}...")
        await client.set_input(input_name)
        print(f"✓ Successfully switched to {input_name}!")

    except Exception as e:
        print(f"✗ Error: {e}")
        if "pairing" in str(e).lower() or "prompt" in str(e).lower():
            print("\n⚠ PAIRING REQUIRED:")
            print("  1. Make sure your LG TV and Mac are on the same WiFi network")
            print("  2. Enable 'LG Connect Apps' on your TV:")
            print("     Settings → Connection → LG Connect Apps → Enable")
            print("  3. Accept the prompt on your TV screen")
        return False
    finally:
        await client.disconnect()

    return True


async def list_inputs(tv_ip: str):
    """List all available inputs on the TV"""

    # Create key storage
    store = SimpleKeyStore()

    # Get stored key
    client_key = await store.get_key(tv_ip)
    if client_key:
        print(f"✓ Using saved credentials")
    else:
        print(f"⚠ First time connection - pairing required")
        print(f"  Accept the prompt on your TV!")

    # Create client with storage
    client = WebOsClient(tv_ip, client_key=client_key, storage=store)

    try:
        print(f"Connecting to {tv_ip}...")
        await client.connect()
        print(f"✓ Connected!")

        inputs = await client.get_inputs()
        print("\nAvailable inputs on your LG TV:")
        print("=" * 50)
        for inp in inputs:
            input_id = inp.get('appId', 'Unknown')
            label = inp.get('label', 'Unknown')
            print(f"  • {label:20s} → {input_id}")
        print("=" * 50)

    except Exception as e:
        print(f"✗ Error: {e}")
        if "pairing" in str(e).lower():
            print("\n⚠ Accept the pairing prompt on your TV!")
    finally:
        await client.disconnect()


def main():
    if len(sys.argv) < 2:
        print("LG TV Network Input Switcher")
        print("=" * 50)
        print("\nUsage:")
        print(f"  {sys.argv[0]} <tv_ip> list")
        print(f"  {sys.argv[0]} <tv_ip> switch <input>")
        print("\nExamples:")
        print(f"  {sys.argv[0]} 192.168.50.144 list")
        print(f"  {sys.argv[0]} 192.168.50.144 switch HDMI_1")
        print(f"  {sys.argv[0]} 192.168.50.144 switch HDMI_2")
        print("\nFirst-time setup:")
        print("  1. Run 'list' command and accept pairing prompt on TV")
        print("  2. Credentials will be saved automatically")
        print("  3. Future commands won't require pairing")
        sys.exit(1)

    tv_ip = sys.argv[1]
    command = sys.argv[2] if len(sys.argv) > 2 else "list"

    if command == "list":
        asyncio.run(list_inputs(tv_ip))
    elif command == "switch":
        if len(sys.argv) < 4:
            print("ERROR: Please specify input name")
            print(f"Usage: {sys.argv[0]} {tv_ip} switch <input>")
            sys.exit(1)
        input_name = sys.argv[3]
        asyncio.run(switch_input(tv_ip, input_name))
    else:
        print(f"Unknown command: {command}")
        print("Valid commands: list, switch")
        sys.exit(1)


if __name__ == "__main__":
    main()
