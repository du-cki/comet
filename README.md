# Comet

A pretty basic CDN node, written in [Rust](https://rust-lang.org).

## Why?

This is a project I've been fiddling around to learn [rust](https://rust-lang.org). You shouldn't be using this in
production because It's pretty basic and has low security (single-key-authentication) for now, I plan to rectify this and
go for a more secure approach but that might prove tricky depending on the client you're using.

## Setup

The setup has been divided into two parts (and further sub-divided for your convenience), setting up the [`server`](#server)
and setting up the `client`. At any point do you feel confused, Please refer back to this guide. Open an
[`issue`](/issues)/[`discussion`](/discussions) thread if you're still confused.

<details>
<summary>Server</summary>
  
  This guide has been further sub-divided into two parts, running the node and exposing the node. There 
  are a few ways that you can utilize to run the node.

  <!-- <details> -->
  <!--   <summary>🌟 Docker</summary> -->
  <!-- </details> -->
  <details>
    <summary>systemd</summary>

- Start by Compiling the project with
``` sh
SQLX_OFFLINE=true cargo build --release
```

- After it compiles, create a [systemd configuration file](https://www.freedesktop.org/software/systemd/man/systemd-system.conf.html) at `/etc/systemd/system/` with this format:
```service
# /etc/systemd/system/comet.service
[Unit]
Description=comet
After=network-online.target

[Service]
Type=simple
WorkingDirectory=/path/to/comet
ExecStart=/path/to/cargo run # you can get the location of `cargo` by `pwd cargo`
User=faaz
Restart=always
TimeoutStopSec=10

[Install]
WantedBy=multi-user.target
```

- Assuming you've named the file `comet.service`, you can start the service with:

``` sh
(sudo) systemctl start comet
# You can also "enable" the service so it runs on startup, run:
# (sudo) systemctl enable comet
```

- If you wish to get rid of an comet instance, run these commands:

``` sh
(sudo) systemctl stop comet
(sudo) systemctl disable comet
(sudo) rm /etc/systemd/system/comet.service
```
  </details>
  
  #### It can then be exposed to the internet through one of these methods.
  
  <details>
    <summary>🌟 CloudFlare Tunnel</summary>
    This is especially nice if you don't have an public IP. This guide assumes you have a [`cloudflare`](https://www.cloudflare.com/)
    account and the [`cloudflared`](https://github.com/cloudflare/cloudflared#installing-cloudflared) CLI installed & logged in.
    <br>
    <br>
    
- Create a tunnel with:
```sh
cloudflared tunnel create ${NAME} 
```

- Create a `config.yaml` file in the [`.cloudflared` directory](https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/install-and-setup/tunnel-useful-terms/#default-cloudflared-directory) with this format:

```yaml
  url: http://localhost:3000
  tunnel: ${TUNNEL_UUID}
  credentials-file: ${HOME}/.cloudflared/${TUNNEL_UUID}.json # or wherever the credential file was generated to
```

- Then, Assign a [CNAME record](https://en.wikipedia.org/wiki/CNAME_record) that'll be used for your tunnel with

```sh
cloudflared tunnel route dns ${TUNNEL_UUID} ${HOSTNAME} # HOSTNAME could be comet.example.com
```

- You'll then be able to run the tunnel with:

```sh
cloudflared tunnel run
# If for some reason you get an Ingress error when you're visiting a route,
# Try running `cloudflared tunnel --config /path/your-config-file.yaml run` instead.
```

- To run the program forever, take a look at [this guide](https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/install-and-setup/tunnel-guide/local/as-a-service/).
  </details>
###### **more coming soon*
</details>

---

<details>
<summary>Client</summary>
  There are a few different ways to get an client up and ready. <br>
  (Note: These configs are assuming you have default routes setup. If you have changed some routes, Change
  it in the config also to reflect changes.)
  <br>
  
  <details>
  <summary>ShareX</summary>
    Here's a basic config for [ShareX](https://getsharex.com/), put this in a file with the extension
    `sxcu` and switch up the values with `$[...]` in it.
  
  ```json
    {
      "Name": "comet-node",
      "Version": "14.0.0", 
      "DestinationType": "ImageUploader, FileUploader",
      "RequestMethod": "POST",
      "RequestURL": "$[BASE_URL]/upload", 
      "Body": "MultipartFormData",
      "FileFormName": "file",
      "Headers": {
        "Authorization": "$[AUTH_KEY]"
      },
      "URL": "$[BASE_URL]{json:file_url}",
      "DeletionURL": "$[BASE_URL]/delete/{json:deletionUrl}",
      "ErrorMessage": "{json:message}"
    }
  ```
  </details>

  <details>
    <summary>iOS Shortcuts</summary>
    This is a neat way to do stuff if you have an iOS device with shortcuts. 
    Get this [shortcut](...)
  </details>

  <details>
    <summary>HTTP API</summary>
      These are the HTTP routes by default of comet. The routes marked with "🔒" are authenticated routes. 
      <br>
      __**Authentication**__
      <br>

You can authenticate yourself by passing the password (set in the config file) into the `Authorization` key of the header.
<br>
<br> \***\*Routes\*\***

<details>
<summary>`POST` **/upload** 🔒</summary>
The endpoint to upload a file. <br>
**Example request**:

```sh
curl -X POST \
  -H "Authorization: ${PASSWORD}" \
  -H "Content-Type: multipart/form-data" \
  -F "file=@/path/to/file" \
  http://localhost:3000
```

        **Example Response**:

```jsonc
HTTP/1.1 200 OK
Content-Type: application/json

{
  "file": "xyz",
  "file_url": "/media/xyz.jpeg",
  "file_size": "812323"
}
```

</details>
<details>
<summary>`DELETE` **/delete/:media_id** [🔒] delete uploaded files.</summary>
The endpoint to delete a previously uploaded file.
<br><br>
**Parameters**: <br> - `media_id` - the UID of the file to delete.
<br>

        **Example request**:

```sh
curl -X DELETE \
  -H "Authorization: ${PASSWORD}" \
  http://localhost:3000/delete/xyz # note: file must not have an file extension.
```

        **Example Response**:

```jsonc
HTTP/1.1 200 OK
Content-Type: application/json

{
  "message": "Removed.",
}
```

</details>
<details>
<summary>`GET` **/media/:media_id** - to get uploaded files.</summary>
The endpoint to get uploaded files.
<br><br>
**Parameters**: <br> - `media_id` - the UID of the file to retrieve.

        **Example request**:

```sh
curl -X GET \
  http://localhost:3000/media/xyz.jpeg
```

        **Example Response**:

```jsonc
HTTP/1.1 200 OK
Content-Type: image/jpeg
Content-Length: [length of image file in bytes]

[Binary image data]
```
        <br>
      </details>
  </details>
</details>
</details>

## Honorable mentions

- This service does not do any sort of media optimization/compression as if now, however this is planned for the future.
- You can save alot of bandwith and make it more snappier if you have a caching layer, I.E cloudflare caching the images.

## Development

If you wish to work with this project, you'll have to do an offline compilation with `SQLX_OFFLINE=true cargo run`, 
to create the initial tables, after which you can create an `.env` file to put `DATABASE_URL` as `sqlite://data.db`
to get compile time checks.

If you've made changes to the SQL queries, you'll have to regenerate the `sqlx-data.json` file if you'd wish to do an
successful offline compilation again. You can regenerate it with the [`sqlx cli`](https://github.com/launchbadge/sqlx) 
through `cargo sqlx prepare`

## License

This project is following the [MIT license](/blob/main/LICENSE).
