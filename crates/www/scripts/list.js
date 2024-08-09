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
 * @returns An array of inserted rows.
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

    // Name, description and region sort
    e_name_header.addEventListener("click", async (e) => alphabetic_sort(data, e.target), false);
    e_description_header.addEventListener("click", async (e) => alphabetic_sort(data, e.target), false);
    e_region_header.addEventListener("click", async (e) => alphabetic_sort(data, e.target), false);
}

/**
 * Alphabetically sort the `data` list based on the "order" attribute on `element`. 
 * 
 * @param {Array<Array<Object>>} data - Takeoff data and their nodes.
 * @param {HTMLElement} element 
 */
function alphabetic_sort(data, element) {
    const prevOrder =  element.getAttribute("order") || "asc";
    element.setAttribute("order", prevOrder === "asc" ? "desc" : "asc");

    data.sort((a, b) => prevOrder === "asc" ? a[0].name > b[0].name : a[0].name < b[0].name);
    for (let i = 0; i < data.length; i++) {
        const [_, e_takeoff] = data[i];
        e_takeoff.style.order = i;
    }
}
