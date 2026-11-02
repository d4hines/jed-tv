Launcher logo source images

The build-only tools/make-menu-tiles.py composes these source images into
288x162 RGB24 files under assets/build/ (or the Nix media output). The TV
receives only youtube.rgb and doom.rgb. Lean draws the caption separately.
The bottom 36 pixels are reserved for that foreground caption.

YouTube: official red play icon from Google's static branding host.
Source: https://www.gstatic.com/youtube/img/branding/favicon/favicon_144x144.png
Brand owner: YouTube / Google.
SHA256: 8639db0dbaa462e7bc11d7884c3a6cb84275dc988811cd2bfaf5cf2e65bc0fe0

Doom: id Software wordmark, sourced from Wikimedia Commons.
Source: https://upload.wikimedia.org/wikipedia/commons/c/cb/Doom_logo.png
Details: https://commons.wikimedia.org/wiki/File:Doom_logo.png
The Commons description identifies id Software as author and labels the
simple text logo public domain. The Doom name and logo remain trademarks
of their owners. The build preserves the alpha silhouette and makes it white
for contrast on the dark background.
SHA256: eec22989eceeaafa6db5717e1ac1ca996cbc79576d636ee7c448318accd49845
