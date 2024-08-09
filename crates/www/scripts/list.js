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
    const out = takeoffs.map((takeoff) => {
        const e_takeoff_container = document.createElement("div");
        const e_takeoff_name = document.createElement("a");
        const e_takeoff_description = document.createElement("span");
        const e_takeoff_region = document.createElement("span");
        const e_takeoff_location = document.createElement("span");

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

    // Guard against missing elements
    const required_elements = [e_search];
    if (required_elements.includes(null)) throw new Error("missing HTML elements");

    // Event handlers
    e_search.addEventListener("input", async (e) => update(data, e.target.value), false);

    // Sort function
    async function update(data, search_text) {
        for (let [takeoff, e_takeoff] of data) {
            const formatText = (text) => text.toLowerCase().replaceAll(' ', '');
            const checkMatch = (text, search) => formatText(text).includes(formatText(search));
            const match = checkMatch(takeoff.name + takeoff.description + takeoff.region, search_text);

            e_takeoff.setAttribute("match", match);
        }
    };
}
