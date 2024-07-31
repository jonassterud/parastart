"use strict"

try {
    const params = new URL(document.location.toString()).searchParams;
    const id = params.get("id");

    if (id !== null) load_takeoff(id);
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
    console.log(takeoff);
}
