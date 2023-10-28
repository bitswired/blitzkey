import json
import os
import platform
import subprocess
import urllib.request

GITHUB_REPO = "bitswired/blitzkey"


def download_file(url, destination):
    with urllib.request.urlopen(url) as response, open(destination, "wb") as out_file:
        out_file.write(response.read())


def get_latest_release(repo):
    url = f"https://api.github.com/repos/{repo}/releases/latest"
    with urllib.request.urlopen(url) as response:
        data = json.load(response)
        return data


def is_blitzkey_installed():
    binary_path = os.path.expanduser("~/blitzkey")
    try:
        output = subprocess.check_output(
            [binary_path, "-V"], stderr=subprocess.STDOUT
        ).decode()
        installed_version = output.split()[-1]
        return installed_version
    except subprocess.CalledProcessError:
        return None
    except FileNotFoundError:
        return None


def main():
    installed_version = is_blitzkey_installed()
    release_data = get_latest_release(GITHUB_REPO)
    latest_version = release_data["tag_name"].lstrip("v")
    name_to_url = {
        asset["name"]: asset["browser_download_url"] for asset in release_data["assets"]
    }

    system = platform.system()
    architecture = platform.machine()

    if system == "Windows":
        file_name = "blitzkey-amd64.exe"
    elif system == "Darwin":
        if architecture == "x86_64":
            file_name = "blitzkey-darwin-amd64"
        else:
            file_name = "blitzkey-darwin-arm64"
    elif system == "Linux":
        if architecture == "x86_64":
            file_name = "blitzkey-linux-amd64"
        else:
            file_name = "blitzkey-linux-arm64"
    else:
        print("Unsupported platform")
        return

    url = name_to_url.get(file_name)
    if not url:
        print(f"Could not find a release for {system} {architecture}")
        return
    destination = os.path.expanduser("~/blitzkey")

    if not installed_version:
        should_install = (
            input("Blitzkey is not installed. Would you like to install it? [Y/n] ")
            .strip()
            .lower()
            or "y"
        )
        if should_install == "y":
            download_file(url, destination)
            print(f"Downloaded {file_name} to {destination}")
    elif installed_version < latest_version:
        should_update = (
            input(
                f"A newer version of Blitzkey (v{latest_version}) is available. Would you like to update? [Y/n] "
            )
            .strip()
            .lower()
            or "y"
        )
        if should_update == "y":
            download_file(url, destination)
            print(f"Updated Blitzkey to v{latest_version} at {destination}")
    else:
        print("Blitzkey is up-to-date.")

    # Make the file executable (for macOS and Linux)
    if system != "Windows":
        os.chmod(destination, 0o755)


if __name__ == "__main__":
    main()
