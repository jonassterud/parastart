"use strict"

window.onload = () => {
    try {
        init_map();
    } catch (error) {
        console.error(error);
    }
}

async function init_map() {
    // Create map
    const map = L.map('map')
        .setView([59.911491, 10.757933], 13)
        .locate({ setView: true })
        .on('locationfound', (_) => {
            console.log("location found"); // todo: inaccurate
        })
        .on('locationerror', (e) => {
            console.error(e);
        });

    // Set layer
    // see also: https://www.maptoolkit.com/doc/tileserver/leaflet/
    L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
        minZoom: 3,
        maxZoom: 19,
        attribution: '&copy; <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>'
    }).addTo(map);

    // Fetch takeoffs
    const takeoffs = await fetch_all_takeoffs_prefer_local(["name", "latitude", "longitude", "wind_dirs"]);
    // Create marker clusters
    const markers = L.markerClusterGroup();
    for (let takeoff of takeoffs) {
        const icon = L.divIcon({
            className: "marker-icon",
            html: icon_for(takeoff),
            iconSize: L.point(24, 24),
        });
        const marker = L.marker([takeoff.latitude, takeoff.longitude], {
            keyboard: false,
            icon: icon,
        });

        marker.bindPopup(`<a href="/takeoffs?id=${takeoff.id}" target="_blank">${takeoff.name}</a>`);
        markers.addLayer(marker);
    }
    map.addLayer(markers);
}

function icon_for(takeoff) {
    const compass = document.createElement("div");
    const n = document.createElement("div");
    const ne = document.createElement("div");
    const e = document.createElement("div");
    const se = document.createElement("div");
    const s = document.createElement("div");
    const sw = document.createElement("div");
    const w = document.createElement("div");
    const nw = document.createElement("div");

    compass.classList.add("compass");
    n.setAttribute("id", "N");
    ne.setAttribute("id", "NE");
    e.setAttribute("id", "E");
    se.setAttribute("id", "SE");
    s.setAttribute("id", "S");
    sw.setAttribute("id", "SW");
    w.setAttribute("id", "W");
    nw.setAttribute("id", "NW");

    compass.append(n, ne, e, se, s, sw, w, nw);

    [...compass.children].forEach((e) => e.setAttribute("hidden", ""));
    [...compass.children].forEach((e) => {
        if (takeoff.wind_dirs.includes(e.id)) {
            e.removeAttribute("hidden");
        }
    });

    return compass;
}
