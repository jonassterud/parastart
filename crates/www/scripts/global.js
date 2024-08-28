"use strict"

/**
 * Fetch takeoffs.
 * 
 * @param {Number} [page] - Optional page offset (`=>1`).
 * @param {Number} [limit] - Optional number of takeoffs to fetch.
 * @param {String} [region] - Optional name of region.
 * @param {Array<String>} [fields] Optional list of columns to fetch.
 * @returns {Promise<Array<Object>>} A list of takeoffs as objects.
 */
async function fetch_takeoffs(page, limit, region, fields) {
    try {
        const url = new URL(window.location.origin);
        url.pathname = "/api/v0/takeoffs";
        
        if (page !== undefined) url.searchParams.append("page", page);
        if (limit !== undefined) url.searchParams.append("limit", limit);
        if (region !== undefined) url.searchParams.append("region", region);
        fields?.forEach((field) => url.searchParams.append("fields", field));

        const response = await fetch(url);
        const out = response.json();

        return out;
    } catch (error) {
        throw error;
    }
}

/**
 * Fetch takeoff.
 * 
 * @param {Number} id - Takeoff id.
 * @returns {Object} A takeoff object.
 */
async function fetch_takeoff(id) {
    try {
        let path = `/api/v0/takeoffs?id=${id}`;

        const response = await fetch(path);
        const json = await response.json();
        const takeoff = json.at(0);

        if (takeoff === undefined) {
            throw new Error("takeoff is undefined");
        }

        return takeoff;
    } catch (error) {
        throw error;
    }
}

/**
 * Fetch all takeoffs - locally if available, remotely if not.
 * 
 * @param {Array<String>} [fields] Optional list of columns to fetch (doesn't support `image`).
 * @returns {Promise<Array<Object>>} A list of takeoffs as objects.
 */
async function fetch_all_takeoffs_prefer_local(fields) {
    let out = [];

    // Check fields
    if (fields === undefined) fields = [];
    if (fields.includes("image")) throw new Error("unsupported field");
    if (!fields.includes("id")) fields.push("id");

    // Calculate hash of remotely stored takeoffs
    const hash = await fetch_takeoffs(undefined, undefined, undefined, ["id"])
        .then((res) => res.map((v) => v.id).join(''))
        .then((s => cyrb53(s).toString()));

    // Remote fetch and update local storage function
    const get_remote = async () => {
        const takeoffs = await fetch_takeoffs(undefined, undefined, undefined, fields);
        window.localStorage.setItem("takeoffs", JSON.stringify(takeoffs));
        window.localStorage.setItem("hash", cyrb53(takeoffs.map((v) => v.id).join('')).toString());

        return takeoffs;
    };

    // Fetch remotely if hash is different, get local if not
    if (hash !== window.localStorage.getItem("hash")) {
        out = await get_remote();
    } else {
        out = JSON.parse(window.localStorage.getItem("takeoffs"));

        // Fetch remotely if missing any fields
        l1: for (let takeoff of out) {
            l2: for (let field of fields) {
                if (takeoff[field] === null) {
                    out = await get_remote();
                    break l1;
                }
            }
        }
    }

    return out;
}

/**
 * Get the location in latitude and longitude, either from cache or current.
 * 
 * @param {Number} cache_time_ms - Number of milliseconds time offset to prefer cache. Defaults to 1 hour.
 * @returns {Promise<Object>} The location.
 */
async function get_location(cache_time_limit=3600000) {
    const location = JSON.parse(window.localStorage.getItem("location")) || {
        latitude: null,
        longitude: null,
        timestamp: null,
    };

    return new Promise((resolve, reject) => {
        if (location?.timestamp !== null && location.timestamp + cache_time_limit >= Date.now()) return resolve(location);
        if (navigator.geolocation === null) return reject(new Error("location API not available"));

        navigator.geolocation.getCurrentPosition((pos) => {
            console.log(pos);
            location.latitude = pos.coords.latitude;
            location.longitude = pos.coords.longitude;
            location.timestamp = pos.timestamp;
            window.localStorage.setItem("location", JSON.stringify(location));

            return resolve(location);
        }, (error) => {
            return reject(error)
        }, {
            enableHighAccuracy: true
        });
    });
}

/**
 * Hash a string.
 * 
 * @param {String} str - A string.
 * @param {String} seed - A seed.
 * @returns A hash.
 * @copyright https://stackoverflow.com/a/52171480
 */
function cyrb53(str, seed = 0) {
    let h1 = 0xdeadbeef ^ seed, h2 = 0x41c6ce57 ^ seed;
    for(let i = 0, ch; i < str.length; i++) {
        ch = str.charCodeAt(i);
        h1 = Math.imul(h1 ^ ch, 2654435761);
        h2 = Math.imul(h2 ^ ch, 1597334677);
    }
    h1  = Math.imul(h1 ^ (h1 >>> 16), 2246822507);
    h1 ^= Math.imul(h2 ^ (h2 >>> 13), 3266489909);
    h2  = Math.imul(h2 ^ (h2 >>> 16), 2246822507);
    h2 ^= Math.imul(h1 ^ (h1 >>> 13), 3266489909);
  
    return 4294967296 * (2097151 & h2) + (h1 >>> 0);
};
