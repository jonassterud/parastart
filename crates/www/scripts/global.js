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
