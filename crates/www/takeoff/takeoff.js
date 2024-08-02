"use strict"

try {
    const params = new URL(document.location.toString()).searchParams;
    const id = params.get("id");

    if (id !== null) {
        load_takeoff(id).then(display_takeoff);
    }
} catch (error) {
    console.error(error);
}

/**
 * Load takeoff.
 * 
 * @param {Number} id - Takeoff ID.
 */
async function load_takeoff(id) {
    const takeoff = await fetch_takeoff(id);

    return takeoff;
}

/**
 * Fill HTML elements with takeoff data.
 * 
 * @param {Object} takeoff - Takeoff object.
 */

async function display_takeoff(takeoff) {
    // Get HTML elements
    const e_name = document.getElementById("name");
    const e_region = document.getElementById("region");
    const e_image = document.getElementById("image");
    const e_description = document.getElementById("description");
    const e_updated = document.getElementById("updated");
    const e_created = document.getElementById("created");
    const e_source_url = document.getElementById("source-url");
    const e_windy_iframe = document.getElementById("windy-iframe");

    // Guard against missing elements
    if ([e_name, e_region, e_image, e_description, e_updated, e_created, e_source_url, e_windy_iframe].includes(null)) {
        throw new Error("failed finding one or more HTML elements");
    }

    // Set text contents
    e_name.innerText = takeoff.name;
    e_region.innerText = takeoff.region;
    e_description.innerText = takeoff.description;
    e_updated.innerText = takeoff.updated;
    e_created.innerText = takeoff.created;
    e_source_url.setAttribute("href", takeoff.source_url);

    // Create image
    if (takeoff.image !== null) {
        const image_base64 = btoa(String.fromCharCode.apply(null, new Uint8Array(takeoff.image)));
        e_image.src = `data:image/png;base64,${image_base64}`;
        e_image.removeAttribute("hidden");
    }

    // Configure compass
    takeoff.wind_dirs.forEach((dir) => {
        const dir_e = document.querySelector(`#compass #${dir}`);
        dir_e.removeAttribute("hidden");
    });

    // Configure Windy frame
    e_windy_iframe.src = `https://embed.windy.com/embed.html?type=map&location=coordinates
    &metricRain=mm&metricTemp=°C&metricWind=m/s&zoom=5&overlay=wind&product=ecmwf&level=200m
    &lat=${takeoff.latitude}&lon=${takeoff.longitude}&detailLat=${takeoff.latitude}&detailLon=${takeoff.longitude}
    &detail=true&message=true`;

    console.log(takeoff);
}
