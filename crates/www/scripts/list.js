"use strict"

async function fill_results() {
    // Get HTML elements
    const e_results_container = document.getElementById("results-container");

    // Guard against missing elements
    const required_elements = [e_results_container];
    if (required_elements.includes(null)) throw new Error("missing HTML elements");

    // Fetch (all) takeoffs (if missing any)
    const hash = await fetch_takeoffs(undefined, undefined, undefined, ["id"])
        .then((res) => res.map((v) => v.id).join(''))
        .then((s => cyrb53(s).toString()));

    if (hash !== window.localStorage.getItem("hash")) {
        const takeoffs = await fetch_takeoffs(undefined, undefined, undefined, ["id", "name", "region", "latitude", "longitude"]);
        window.localStorage.setItem("takeoffs", JSON.stringify(takeoffs));
        window.localStorage.setItem("hash", cyrb53(takeoffs.map((v) => v.id).join('')).toString());
        console.log("fetching")
    }


    //return takeoffs;
}
