"use strict"

window.onload = () => {
    try {
        fill_results_table().then(handle_sorting);
    } catch (error) {
        console.error(error);
    }
}

/**
 * Fill the "results-table" with takeoffs.
 * 
 * @returns An array of arrays, where the innermost array contains a takeoff object and its node.
 */
async function fill_results_table() {
    // Get HTML elements
    const e_results_content = document.getElementById("results-content");

    // Guard against missing elements
    const required_elements = [e_results_content];
    if (required_elements.includes(null)) throw new Error("missing HTML elements");

    // Fetch (all) takeoffs (if missing any)
    let takeoffs = [];
    const hash = await fetch_takeoffs(undefined, undefined, undefined, ["id"])
        .then((res) => res.map((v) => v.id).join(''))
        .then((s => cyrb53(s).toString()));

    if (hash !== window.localStorage.getItem("hash")) {
        takeoffs = await fetch_takeoffs(undefined, undefined, undefined, ["id", "name", "description", "region", "latitude", "longitude"]);
        window.localStorage.setItem("takeoffs", JSON.stringify(takeoffs));
        window.localStorage.setItem("hash", cyrb53(takeoffs.map((v) => v.id).join('')).toString());
    } else {
        takeoffs = JSON.parse(window.localStorage.getItem("takeoffs"));
    }

    // Insert takeoffs
    const out = takeoffs.map((takeoff, i) => {
        const e_takeoff_container = document.createElement("div");
        const e_takeoff_name = document.createElement("a");
        const e_takeoff_description = document.createElement("span");
        const e_takeoff_region = document.createElement("span");
        const e_takeoff_location = document.createElement("span");
        const e_takeoff_distance = document.createElement("span");

        e_takeoff_container.style.order = i;
        e_takeoff_container.classList.add("takeoff");
        e_takeoff_name.classList.add("name");
        e_takeoff_description.classList.add("description");
        e_takeoff_region.classList.add("region");
        e_takeoff_location.classList.add("location");
        e_takeoff_distance.classList.add("distance");

        e_takeoff_name.innerText = takeoff.name;
        e_takeoff_name.href = `?id=${takeoff.id}`;
        e_takeoff_description.innerText = takeoff.description; //.split(' ').slice(0, 10).join(' ').substring(0, 10 * 9) + " (...)";
        e_takeoff_region.innerText = takeoff.region;
        e_takeoff_location.innerText = `${takeoff.latitude.toFixed(4)}, ${takeoff.longitude.toFixed(4)}\n`;
        e_takeoff_distance.innerText = "";

        e_takeoff_location.append(e_takeoff_distance);
        e_takeoff_container.append(e_takeoff_name, e_takeoff_description, e_takeoff_region, e_takeoff_location);
        e_results_content.append(e_takeoff_container);

        return [takeoff, e_takeoff_container];
    });

    return out;
}

/**
 * Handle sorting.
 * 
 * @param {Array<Array<Object>>} data - An array of arrays, where the innermost array contains a takeoff object and its node.
 */
function handle_sorting(data) {
    // Get HTML elements
    const e_search = document.getElementById("search");
    const e_name_header = document.getElementById("name-header");
    const e_description_header = document.getElementById("description-header");
    const e_region_header = document.getElementById("region-header");
    const e_location_header = document.getElementById("location-header");

    // Guard against missing elements
    const required_elements = [e_search, e_name_header, e_description_header, e_region_header, e_location_header];
    if (required_elements.includes(null)) throw new Error("missing HTML elements");

    // Search sort
    e_search.addEventListener("input", async (e) => {
        for (let [takeoff, e_takeoff] of data) {
            // Text match
            const formatText = (text) => text.toLowerCase().replaceAll(' ', '');
            const checkMatch = (text, search) => formatText(text).includes(formatText(search));
            const match = checkMatch(takeoff.name + takeoff.description + takeoff.region, e.target.value);

            e_takeoff.setAttribute("match", match);  
        }
    });

    // Name, description, region and location sort
    try {
        e_name_header.addEventListener("click", async (e) => alphabetic_sort(data, e.target, (v) => v.name), false);
        e_description_header.addEventListener("click", async (e) => alphabetic_sort(data, e.target, (v) => v.description), false);
        e_region_header.addEventListener("click", async (e) => alphabetic_sort(data, e.target, (v) => v.region), false);
        e_location_header.addEventListener("click", async (e) => location_sort(data, e.target), false);
    } catch (error) {
        console.error(error);
    }
}

/**
 * Alphabetically sort the `data` list based on the "order" attribute on `element`. 
 * 
 * @param {Array<Array<Object>>} data - Takeoff data and their nodes.
 * @param {HTMLElement} element - HTML element that was clicked.
 * @param {(a: Array<Object>) => String} fn - Function that returns the parameter to sort by.
 */
function alphabetic_sort(data, element, fn) {
    const prevOrder =  element.getAttribute("order") || "asc";
    element.setAttribute("order", prevOrder === "asc" ? "desc" : "asc");
    
    const collator = new Intl.Collator();
    data.sort((a, b) => collator.compare(fn(a[0]), fn(b[0])));

    for (let i = 0; i < data.length; i++) {
        if (prevOrder === "desc") {
            data[i][1].style.order = i;
        } else if (prevOrder === "asc") {
            data[data.length - 1 - i][1].style.order = i;
        } else {
            throw new Error(`unexpected prevOrder: ${prevOrder}`);
        }
    }
}

/**
 * Geographically sort the `data` list based on the "order" attribute on `element`
 * 
 * @param {Array<Array<Object>>} data - Takeoff data and their nodes.
 * @param {HTMLElement} element - HTML element that was clicked.
 * @copyright https://stackoverflow.com/a/21623206
 */
function location_sort(data, element) {
    const prevOrder =  element.getAttribute("order") || "asc";
    element.setAttribute("order", prevOrder === "asc" ? "desc" : "asc");
    
    // TODO: Re-use distance and update based on timestamp
    if (navigator.geolocation) {
        navigator.geolocation.getCurrentPosition((pos) => {
            // Distance formula
            const r = 6371;
            const p = Math.PI / 180;
            const dist = (a, b) => {
                const n = 0.5 - Math.cos((b.latitude - a.latitude) * p) / 2
                + Math.cos(a.latitude * p) * Math.cos(b.latitude * p) *
                  (1 - Math.cos((b.longitude - a.longitude) * p)) / 2;

                return 2 * r * Math.asin(Math.sqrt(n));
            };

            // Map the distance
            data = data.map((v) => {
                const distance = dist(v[0], pos.coords);
                v[0]["distance"] = distance

                return v;
            });

            // Sort and apply order
            data.sort((a, b) => a[0].distance - b[0].distance);
            for (let i = 0; i < data.length; i++) {
                if (prevOrder === "desc") {
                    data[i][1].style.order = i;
                } else if (prevOrder === "asc") {
                    data[data.length - 1 - i][1].style.order = i;
                } else {
                    throw new Error(`unexpected prevOrder: ${prevOrder}`);
                }

                // Display distance
                const e_location = data[i][1].getElementsByClassName("distance").item(0);
                if (e_location === null) throw new Error("missing HTML element");
                e_location.innerText = `Ca. ${data[i][0].distance.toFixed(2)} km`;
            }
        }, (error) => {
            console.error(error);
        });
    }
}
