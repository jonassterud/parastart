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

        e_takeoff_container.style.order = i;
        e_takeoff_container.classList.add("takeoff");
        e_takeoff_name.classList.add("name");
        e_takeoff_description.classList.add("description");
        e_takeoff_region.classList.add("region");
        e_takeoff_location.classList.add("location");

        e_takeoff_name.innerText = takeoff.name;
        e_takeoff_name.href = `?id=${takeoff.id}`;
        e_takeoff_description.innerText = takeoff.description; //.split(' ').slice(0, 10).join(' ').substring(0, 10 * 9) + " (...)";
        e_takeoff_region.innerText = takeoff.region;
        e_takeoff_location.innerText = "todo"; // `${takeoff.latitude}, ${takeoff.longitude}`;

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
        e_name_header.addEventListener("click", async (e) => alphabetic_sort(data, e.target, (a, b) => a.name > b.name), false);
        e_description_header.addEventListener("click", async (e) => alphabetic_sort(data, e.target, (a, b) => a.description > b.description), false);
        e_region_header.addEventListener("click", async (e) => alphabetic_sort(data, e.target, (a, b) => a.region > b.region), false);
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
 * @param {(a: Array<Object>, b: Array<Object>) => Boolean} fn - Ascending sort function for `data`.
 */
function alphabetic_sort(data, element, fn) {
    const prevOrder =  element.getAttribute("order") || "asc";
    element.setAttribute("order", prevOrder === "asc" ? "desc" : "asc");
    data.sort((a, b) => fn(a[0], b[0]));

    for (let i = 0; i < data.length; i++) {
        if (prevOrder === "asc") {
            data[i][1].style.order = i;
        } else if (prevOrder === "desc") {
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

            // Sort
            data.sort((a, b) => dist(a, pos.coords) < dist(b, pos.coords));
            
            // Apply order
            for (let i = 0; i < data.length; i++) {
                if (prevOrder === "asc") {
                    data[i][1].style.order = i;
                } else if (prevOrder === "desc") {
                    data[data.length - 1 - i][1].style.order = i;
                } else {
                    throw new Error(`unexpected prevOrder: ${prevOrder}`);
                }
            }
        }, (error) => {
            console.error(error);
        });
    }
}
