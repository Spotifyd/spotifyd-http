# Spotifyd-http

This web server remote controls Spotify Connect devices via HTTP requests.

## Methods

These are the currently supported methods:

### GET /devices
Returns a list of device ID/Name pairs.

### PUT /device_id/{play, pause, next, prev}
Plays, pauses, skips, and returns to previous song.

### {GET, PUT, POST} /device_id/tracks
Gets, replaces, or appends tracks to the playlist. The `PUT` and `POST` take
one or more `id` parameters. Example:
```bash
TRACKS="id=2BhU0Hl5OatWiCW93pE2b8&id=731OW49heGHCMrMOREHYlY&id=6zAPaRDoT99ThFtIXUJwhO"
curl -X POST -d "$TRACKS" 127.0.0.1:6767/device_id/tracks
```
