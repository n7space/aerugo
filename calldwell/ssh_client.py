"""Module containing classes for SSH management."""

from dataclasses import dataclass
from typing import Optional

import paramiko
from paramiko.channel import ChannelFile, ChannelStderrFile, ChannelStdinFile


class SSHClient:
    """Convenience class for all SSH-related functionality."""

    def __init__(self, host: str, login: str, password: str, port: int = 22) -> None:
        """Connects with SSH server. After connecting, your working dir will be set to home dir of
        logged in user (if it has one).

        # Parameters:
        * `host` - server hostname (can be an IP address)
        * `login` - user login
        * `password` - user password
        * `port` - SSH port, default: 22
        """
        self.client: paramiko.SSHClient = paramiko.SSHClient()
        self.client.load_system_host_keys()
        self.client.connect(hostname=host, port=port, username=login, password=password)
        self.sftp: paramiko.SFTPClient = self.client.open_sftp()

    @dataclass
    class CommandChannels:
        """Channels of executed SSH command"""

        stdin: ChannelStdinFile
        stdout: ChannelFile
        stderr: ChannelStderrFile

    def execute(
        self,
        command: str,
        timeout: Optional[float] = None,
        environment: Optional[dict[str, str]] = None,
    ) -> CommandChannels:
        """Executes a command on remote, returns `stdin`, `stdout`, `stderr` wrapped in dataclass.

        # Parameters
        * `command` - command (and arguments) to be executed, in form of a single string
        * `timeout` - time, in seconds, to wait for pending I/O operation before an exception is
                      raised, or `None` to disable timeout.
        * `environment` - additional environment variables for executed program.
        """

        stdin, stdout, stderr = self.client.exec_command(
            command, timeout=timeout, environment=environment
        )
        return self.CommandChannels(stdin, stdout, stderr)

    def upload_file_to_remote(self, local_source_path: str, remote_destination_path: str) -> None:
        """Sends a file from local machine to remote. Will raise an exception on failure.

        # Parameters
        * `local_source_path` - path to local file that will be uploaded
        * `remote_destination_path` - path on remote machine where the file will be uploaded
        """
        self.sftp.put(local_source_path, remote_destination_path, confirm=True)

    def download_file_from_remote(
        self, remote_source_path: str, local_destination_path: str
    ) -> None:
        """Downloads a file from remote to local machine. Will raise an exception on failure.

        # Parameters
        * `remote_source_path` - remote path to downloaded file
        * `local_destination_path` - local destination path for downloaded file
        """
        self.sftp.get(remote_source_path, local_destination_path)

    def close(self) -> None:
        """Close the connection. Better do that manually after finishing up with SSH, to prevent
        halting the program."""
        self.client.close()
