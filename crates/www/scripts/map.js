"use strict"

window.onload = () => {
    try {
        init_map();
    } catch (error) {
        console.error(error);
    }
}

async function init_map() {
    // Get cached location if any, and create map
    const map = L.map('map').setView([59.911491, 10.757933], 13);

    // Override the "getCurrentPosition" function 
    navigator.geolocation.getCurrentPosition = (success, _) => {
        success({
            coords: {
                latitude: 45.4111,
                longitude: -75.6981,
            },
            timestamp: Date.now(),
        });
    }

    // Add "locate" plugin
    const locate = L.control.locate({
        locateOptions: {
            enableHighAccuracy: true,
            returnToPrevBounds: true,
        },
        strings: {
            popup: `Wrong? <a href="/settings">Set location</a>`
        },
    }).addTo(map);
    
    // Override position for "locate" plugin if configured
    // todo

    // Set layer
    // see also: https://www.maptoolkit.com/doc/tileserver/leaflet/
    L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
        minZoom: 3,
        maxZoom: 19,
        attribution: '&copy; <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>'
    }).addTo(map);

    // Fetch takeoffs and create markers
    const markers = L.markerClusterGroup();
    const takeoffs = await fetch_all_takeoffs_prefer_local(["name", "latitude", "longitude", "wind_dirs"]);
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
    compass.classList.add("compass");

    const dirs = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
    for (let dir of dirs) {
        const e = document.createElement("div");
        e.setAttribute("id", dir);
        compass.append(e)
    }

    [...compass.children].forEach((e) => {
        if (takeoff.wind_dirs.includes(e.id)) {
            e.removeAttribute("hidden");
        } else {
            e.setAttribute("hidden", "");
        }
    });

    return compass;
}
