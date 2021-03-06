#!/bin/sh
#
# Download and install the latest Munki 3 tools from munkibuilds.org

cat <<EOF

**************************
** Munki auto-installer **
**************************

EOF

if [ "$(id -u)" -ne 0 ]; then
    echo "(Please enter your sudo password when prompted)"
    echo ""
fi

tmpdir=$(mktemp -d /tmp/munkibuilds-XXXX)
pkg_download="${tmpdir}/munki3.pkg"
echo "Grabbing the latest version..."
curl \
curl    -s \
    -f \
    -o "${pkg_download}" \
    --connect-timeout 30 \
    https://munkibuilds.org/munkitools3-latest.pkg

sudo /usr/sbin/installer -pkg "${pkg_download}" -target /

salinstall=$(curl -L https://github.com/salopensource/sal-scripts/releases/latest |grep pkg|grep -o '"[^"]*"' |grep sal)

sudo /usr/sbin/installer -pkg "${salinstall}" -target /





