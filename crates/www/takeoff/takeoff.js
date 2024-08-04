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
    const e_altitude = document.getElementById("altitude");
    const e_altitude_diff = document.getElementById("altitude-diff");
    const e_updated = document.getElementById("updated");
    const e_created = document.getElementById("created");
    const e_source_url = document.getElementById("source-url");
    const e_windy_iframe = document.getElementById("windy-iframe");
    const e_windy_height = document.getElementById("windy-height");
    const e_readable_height = document.getElementById("readable-height");

    // Guard against missing elements
    const required_elements = [
        e_name, e_region, e_image, e_description, e_updated,
        e_created, e_source_url, e_windy_iframe, e_windy_height,
        e_readable_height, e_altitude, e_altitude_diff
    ];

    if (required_elements.includes(null)) {
        throw new Error("failed finding one or more HTML elements");
    }

    // Set text contents
    e_name.innerText = takeoff.name;
    e_region.innerText = takeoff.region;
    e_description.innerText = takeoff.description;
    e_altitude.innerText = takeoff.altitude;
    e_altitude_diff.innerText = takeoff.altitude_diff;
    e_updated.innerText = takeoff.updated;
    e_created.innerText = takeoff.created;
    e_source_url.setAttribute("href", takeoff.source_url);

    // Create image
    if (takeoff.image !== null) {
        const image_base64 = btoa(String.fromCharCode.apply(null, new Uint8Array(takeoff.image)));
        e_image.src = `data:image/png;base64,${image_base64}`;
        e_image.removeAttribute("hidden");
        e_image.addEventListener("click", () => window.location.href = e_image.src);
    }

    // Configure compass
    takeoff.wind_dirs.forEach((dir) => {
        const dir_e = document.querySelector(`#compass #${dir}`);
        dir_e.removeAttribute("hidden");
    });

    // Set Windy iframe function
    const set_windy_iframe = (latitude, longitude, height) => {
        e_windy_iframe.src = `https://embed.windy.com/embed.html?
        type=map&location=coordinates&metricRain=mm&metricTemp=°C&metricWind=m/s
        &zoom=5&overlay=wind&product=ecmwf&level=${height}&lat=${latitude}&lon=${longitude}
        &detailLat=${latitude}&detailLon=${longitude}&detail=true&message=true`;    
    };

    // Synchronize height slider and Windy iframe function
    const synchronize_windy_slider = () => {
        const windy_height = windy_heights.at(e_windy_height.value);
        const readable_height = readable_heights.at(e_windy_height.value);

        set_windy_iframe(takeoff.latitude, takeoff.longitude, windy_height);
        e_readable_height.innerText = readable_height;

        console.log(element);
    };

    // Configure Windy height slider
    const windy_heights = ["surface", "100m", "950h", "925h", "900h", "850h", "800h", "700h", "600h", "500h", "400h", "300h", "250h", "200h", "150h", "10h"]
    const number_heights = [0, 100, 600, 750, 900, 1500, 2000, 3000, 4200, 5500, 7000, 9000, 10000, 11700, 13500, 30000];
    const readable_heights = ["Surface", "100 m", "600 m", "750 m", "900 m", "1500 m", "2000 m", "3000 m", "4200 m", "5500 m", "7000 m", "9000 m", "10 km", "11,7 km", "13,5 km", "30 km"]
    e_windy_height.addEventListener("change", synchronize_windy_slider);

    // Set best initial height for height slider
    const initial_number_height = number_heights.reduce((prev, curr) => (Math.abs(curr - takeoff.altitude) < Math.abs(prev - takeoff.altitude) ? curr : prev));
    e_windy_height.value = number_heights.indexOf(initial_number_height);
    synchronize_windy_slider();
}
