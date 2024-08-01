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
    const e_name = document.getElementById("name");
    const e_region = document.getElementById("region");
    const e_image_container = document.getElementById("image-container");
    const e_description = document.getElementById("description");
    const e_updated = document.getElementById("updated");
    const e_created = document.getElementById("created");
    const e_source_url = document.getElementById("source-url");

    if ([e_name, e_region, e_image_container, e_description, e_updated, e_created, e_source_url].includes(null)) {
        throw new Error("failed finding one or more HTML elements");
    }

    e_name.innerText = takeoff.name;
    e_region.innerText = takeoff.region;
    e_description.innerText = takeoff.description;
    e_updated.innerText = takeoff.updated;
    e_created.innerText = takeoff.created;
    e_source_url.setAttribute("href", takeoff.source_url);

    if (takeoff.image !== null) {
        const image_base64 = btoa(String.fromCharCode.apply(null, new Uint8Array(takeoff.image)));
        const e_image = document.createElement("img");

        e_image.src = `data:image/png;base64,${image_base64}`; 
        e_image_container.appendChild(e_image);
    }
}
