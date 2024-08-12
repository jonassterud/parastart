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
            console.log("location found");
        })
        .on('locationerror', (e) => {
            console.error(e);
        });

    // Set layer
    L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
        maxZoom: 19,
        attribution: '&copy; <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>'
    }).addTo(map);

    // Fetch takeoffs
    const takeoffs = await fetch_all_takeoffs_prefer_local(["name", "latitude", "longitude"]);

    // Create marker clusters
    const markers = L.markerClusterGroup();
    for (let takeoff of takeoffs) {
        const marker = L.marker([takeoff.latitude, takeoff.longitude], {
            keyboard: false,
        });

        marker.bindPopup(`<a href="/takeoffs?id=${takeoff.id}" target="_blank">${takeoff.name}</a>`);
        markers.addLayer(marker);
    }
    map.addLayer(markers);
}
