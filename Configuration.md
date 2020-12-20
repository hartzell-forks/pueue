The default configuration file of Pueue is located in these directories:

- Linux: `$HOME/.config/pueue/pueue.yml`.
- MacOs: `$HOME/Library/Preferences/pueue/pueue.yml`
- Windows: `%APPDATA%\Local\pueue`

A default configuration file will be generated after starting `pueued` for the first time.
You can also force pueue to use a specific configuration file with the `-c` flag for both the daemon and the client.

```yaml
---
shared:
  pueue_directory: /home/$USER/.local/share/pueue
  use_unix_sockets: true
  unix_sockets_path: /home/$USER/.local/share/pueue/pueue_$USER.socket
  host: "localhost"
  port: "6924"
  daemon_cert: /home/$USER/.local/share/pueue/certs/daemon.cert
  daemon_key: /home/$USER/.local/share/pueue/certs/daemon.key
  shared_secret_path: /home/$USER/.local/share/pueue/shared_secret

client:
  read_local_logs: true
  show_confirmation_questions: false
  show_expanded_aliases: false
  max_status_height: null

daemon:
  default_parallel_tasks: 1
  pause_all_on_failure: false
  pause_group_on_failure: false
  callback: ""Task {{ id }}\nCommand: {{ command }}\nPath: {{ path }}\nFinished with status '{{ result }}'\""
  groups:
    default: 1
```

### Shared

- `pueue_directory` The location Pueue uses for its intermediate files and logs.
- `use_unix_sockets` (Unix only) Whether the daemon should listen on a Unix- or a TCP-socket.
- `unix_socket_path` (Unix only) The path the unix socket is located at.
- `host` The host the daemon listens on and the client connects to. Only used when in TCP mode.
- `port` The port the daemon listens on and the client connects to. Only used when in TCP mode.
- `daemon_cert` The TLS certificate used for encrypting any TCP traffic.
- `daemon_key` The TLS private key used for encrypting any TCP traffic.
- `shared_secret_path` The path to the file, which contains the secret used for authentication with the daemon.

### Client

- `read_local_logs` If the client runs on the same machine as the daemon, logs don't have to be sent via the socket. Instead they can be read directly from the disk.
- `show_confirmation_questions` The client will print warnings that require confirmation for different critical commands.
- `show_expanded_aliases` Determines, whether the original command or the command after expanding any aliases in the `pueue_aliases` file will be shown when calling `pueue status`.
- `max_status_height` [int|null] If a number X is given, all table rows in the `status` subcommand, which have more than X lines will be truncated.

### Daemon

- `default_parallel_tasks` Determines how many tasks should be processed concurrently.
- `pause_on_failure` If set to `true`, the daemon stops starting new task as soon as a single task fails. Already running tasks will continue.
- `callback` The command that will be called after a task finishes. Can be parameterized
- `groups` This is a list of the groups with their amount of allowed parallel tasks.
    It's advised to not manipulate this manually, but rather use the `group` subcommand to create and remove groups and the `parallel` subcommand to change any parallelism settings.
