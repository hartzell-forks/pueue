Running `pueued` on a server and wanting to check on the current progress without a login shell is a common scenario.

Pueue supports TLS encrypted communication to a remote and secret based authentication.
If that's not safe enough for your use-case, you can always listen on unix-sockets/localhost and do port/unix-socket forwarding via SSH.

**Reminder:**

- You have to set `read_local_logs` config to `false` in the client config.
    Otherwise `follow` and `log` won't work.
- You have to set the secret of your client configuration to the same of the remote daemon.

**Tips:**

- It's nice to use a separate configuration file for this.
    The file can be set via the `-c` flag.
    You should also consider creating an shell alias for this.
- You can create a systemd job, whose job is to open the ssh connection and to reconnect, whenever the connection goes away.


## Remote via TCP and TLS

Pueue creates a self-signed TLS certificate at startup.
This will then be used for any TCP communication.

1. Set `use_unix_socket` to `false`.
2. Set your `host` and `port` values on both daemon and client.
3. Restart/start `pueued`
4. Copy the server's `daemon.cert` to the client machine
5. Copy the server's `sharted_secret` to the client machine
6. Update the `daemon_cert` and `sharted_secret_path` to point to those files.

The `pueue` client will only connect to a `pueued` daemon, which serves the known certificate.
That's why your client configuration must point to a copy of your server's `daemon.cert`.


## Forwarding via SSH
### Port forwarding

For port this looks like this:

```bash
ssh -L 127.0.0.1:6924:127.0.0.1:6924 $REMOTE_USER@yourhost
```

You can now connect from your local pueue to the remote pueue via port 6924. Just write `pueue -p 6924 status`.

### Unix Socket forwarding

Unix-socket to unix-socket is of course also possible:

```bash
ssh -L /tmp/local.socket:/home/$REMOTE_USER/.local/share/pueue/pueue_$REMOTE_USER.sock $REMOTE_USER@yourhost
```
Just connect via `pueue -u /tmp/local_socket status`.
