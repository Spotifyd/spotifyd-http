# Spotifyd-http

This web server remote controls Spotify Connect devices via HTTP requests.

## Methods

These are the currently supported methods:

### GET /devices
Returns a list of device ID/Name pairs.

### PUT /device_id/{play, pause, next, prev}
Plays, pauses, skips, and returns to previous song.
