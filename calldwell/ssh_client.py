from dataclasses import dataclass
from typing import Dict, Optional

import paramiko
from paramiko.channel import ChannelFile, ChannelStderrFile, ChannelStdinFile


class SSHClient:
    """Convenience class for all SSH-related functionality."""

    def __init__(self, host: str, login: str, password: str, port: int = 22):
        """Connects with SSH server. After connecting, your working dir will be set to home dir of
        logged in user.

        # Parameters:
        * `host` [str] - server hostname (can be an IP address)
        * `login` [str] - user login
        * `password` [str] - user password
        * `port` [int] - SSH port, default: 22
        """
        self.client = paramiko.SSHClient()
        self.client.load_system_host_keys()
        self.client.connect(hostname=host, port=port, username=login, password=password)
        self.sftp = self.client.open_sftp()

    @dataclass
    class CommandChannels:
        stdin: ChannelStdinFile
        stdout: ChannelFile
        stderr: ChannelStderrFile

    def execute(
        self,
        command: str,
        timeout: Optional[float] = None,
        environment: Optional[Dict[str, str]] = None,
    ) -> CommandChannels:
        """Executes a command on remote, returns `stdin`, `stdout`, `stderr` triple

        # Parameters
        * `command` [str] - command (and arguments) to be executed, in form of a single string
        * `timeout` [Optional[float]] - time, in seconds, to wait for pending I/O operation before
                                        an exception is raised, or `None` to disable timeout.
        * `environment` [Optional[Dict[str, str]]] - additional environment variables for executed
                                                  program.
        """

        stdin, stdout, stderr = self.client.exec_command(
            command, timeout=timeout, environment=environment
        )
        return self.CommandChannels(stdin, stdout, stderr)

    def upload_file_to_remote(self, local_source_path: str, remote_destination_path: str):
        """Sends a file from local machine to remote. Will raise an exception on failure.

        # Parameters
        * `local_source_path` [str] - path to local file that will be uploaded
        * `remote_destination_path` [str] - path on remote machine where the file will be uploaded
        """
        self.sftp.put(local_source_path, remote_destination_path, confirm=True)

    def download_file_from_remote(self, remote_source_path: str, local_destination_path: str):
        """Downloads a file from remote to local machine. Will raise an exception on failure.

        # Parameters
        * `remote_source_path` [str] - remote path to downloaded file
        * `local_destination_path` [str] - local destination path for downloaded file
        """
        self.sftp.get(remote_source_path, local_destination_path)

    def close(self):
        """Close the connection. Better do that manually after finishing up with SSH, to prevent
        halting the program."""
        self.client.close()
