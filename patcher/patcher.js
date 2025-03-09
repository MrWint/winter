// Misc constants
const LITTLE_ENDIAN = true;
const PAGE_SIZE = 512;
const PARAGRAPH_SIZE = 16;

// hard-coded Huffman tree the game uses for encoding lookback locations used during unpacking resources.
const UPPER_BITS = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
    0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
    0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09,
    0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0B, 0x0B, 0x0B, 0x0B, 0x0B, 0x0B, 0x0B, 0x0B,
    0x0C, 0x0C, 0x0C, 0x0C, 0x0D, 0x0D, 0x0D, 0x0D, 0x0E, 0x0E, 0x0E, 0x0E, 0x0F, 0x0F, 0x0F, 0x0F,
    0x10, 0x10, 0x10, 0x10, 0x11, 0x11, 0x11, 0x11, 0x12, 0x12, 0x12, 0x12, 0x13, 0x13, 0x13, 0x13,
    0x14, 0x14, 0x14, 0x14, 0x15, 0x15, 0x15, 0x15, 0x16, 0x16, 0x16, 0x16, 0x17, 0x17, 0x17, 0x17,
    0x18, 0x18, 0x19, 0x19, 0x1A, 0x1A, 0x1B, 0x1B, 0x1C, 0x1C, 0x1D, 0x1D, 0x1E, 0x1E, 0x1F, 0x1F,
    0x20, 0x20, 0x21, 0x21, 0x22, 0x22, 0x23, 0x23, 0x24, 0x24, 0x25, 0x25, 0x26, 0x26, 0x27, 0x27,
    0x28, 0x28, 0x29, 0x29, 0x2A, 0x2A, 0x2B, 0x2B, 0x2C, 0x2C, 0x2D, 0x2D, 0x2E, 0x2E, 0x2F, 0x2F,
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F];
const ADDITIONAL_BITS = [
    0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
    0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
    0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
    0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
    0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
    0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
    0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06,
    0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06,
    0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06,
    0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
    0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
    0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08];


function logError(message) {
    let logger = document.getElementById('log');
    logger.classList.add("logError");
    logger.textContent = message;
}
function logSuccess(message) {
    let logger = document.getElementById('log');
    logger.classList.add("logSuccess");
    logger.textContent = message;
}

function initLog() {
    let logger = document.getElementById('log');
    logger.classList.remove("logError");
    logger.classList.remove("logSuccess");
    logger.textContent = "";
}

/**
 * Converts the name of a resource to its packed version, prepending a "P" to the extension.
 * @param {string} resource_name
 * @returns {string}
 */
function getPackedResourceName(resource_name) {
    const extension_index = resource_name.lastIndexOf(".");
    if (extension_index === -1 || extension_index >= resource_name.length - 1) {
        return "";
    }
    return resource_name.substring(0, extension_index + 1) + "P" + resource_name.substring(extension_index + 1, resource_name.length - 1);
}

/**
 * Decompresses a packed resource by emulating the game's unpacking routine.
 * @param {Uint8Array} packed_resource
 * @returns {Uint8Array | null}
 */
function unpackResource(packed_resource) {
    let data_view = new DataView(packed_resource.buffer, packed_resource.byteOffset, packed_resource.byteLength);
    const unpacked_length = data_view.getUint32(0, LITTLE_ENDIAN);

    let unpacked_array = new Uint8Array(unpacked_length);

    // build initial Huffman tree with 314 tokens (256 byte literals and 58 lookback tokens)
    let weights = new Array(627);
    let left_child_or_value = new Array(627);
    let value_to_index = new Array(314);
    let parent_index = new Array(627);
    for (let i = 0; i < 314; i++) {
        weights[i] = 1;
        left_child_or_value[i] = 627 + i;
        value_to_index[i] = i;
    }
    let left_child_index = 0;
    for (let i = 314; i < 627; i++) {
        weights[i] = weights[left_child_index] + weights[left_child_index+1];
        left_child_or_value[i] = left_child_index;
        parent_index[left_child_index] = i;
        parent_index[left_child_index+1] = i;
        left_child_index += 2;
    }
    parent_index[626] = 0;

    // the packed input is read one bit at a time
    let bit_position = 8*4;
    let readBit = () => {
        let b = (packed_resource[bit_position >> 3] >> (7 - (bit_position & 7))) & 1;
        bit_position += 1;
        return b;
    };
    let readBits = (numBits) => {
        let val = 0;
        for (let i = 0; i < numBits; i++) {
            val = (val << 1) + readBit();
        }
        return val;
    };

    let cur_length = 0;
    while (cur_length < unpacked_length) {
        // Find next token by traversing the Huffman tree
        let si = left_child_or_value[626];
        while (si < 627) {
            si = left_child_or_value[si + readBit()];
        }
        const value = si - 627;

        // Update the Huffman tree by increasing the weight of the current token and rebalancing as necessary
        if (weights[626] == 0xfff0) {
            logError("Unpacked data too long; rebalancing unimplemented");
            return null;
        }
        let index = value_to_index[value];
        while (true) {
            weights[index] += 1;
            let swap_index = index;
            while (swap_index < 626 && weights[index] > weights[swap_index+1]) {
                swap_index += 1;
            }
            if (swap_index !== index) {
                [weights[index], weights[swap_index]] = [weights[swap_index], weights[index]];
                let di2 = left_child_or_value[index];
                if (di2 < 627) {
                    parent_index[di2] = swap_index;
                    parent_index[di2+1] = swap_index;
                } else {
                    value_to_index[di2-627] = swap_index;
                }
                let di3 = left_child_or_value[swap_index];
                if (di3 < 627) {
                    parent_index[di3] = index;
                    parent_index[di3+1] = index;
                } else {
                    value_to_index[di3-627] = index;
                }
                [left_child_or_value[index], left_child_or_value[swap_index]] = [left_child_or_value[swap_index], left_child_or_value[index]];
            }
            index = parent_index[swap_index];
            if (index == 0) { break; }
        }

        // Expand the unpacked bytes based on the decoded token
        if (value < 256) {
            // Token is a byte literal
            unpacked_array[cur_length] = value;
            cur_length += 1;
        } else {
            // Token is a lookback operation
            const bx = readBits(8);
            const upper_bits = UPPER_BITS[bx] << 6;
            const additional_bits = ADDITIONAL_BITS[bx] - 2;
            const middle_bits = (bx << additional_bits) & 0x3f;
            const lower_bits = readBits(additional_bits);
            const lookback = (upper_bits | middle_bits | lower_bits) + 1;
            const length = value - 253;
            for (let i = 0; i < length; i++) {
                unpacked_array[cur_length] = (cur_length >= lookback ? unpacked_array[cur_length - lookback] : 0x20);
                cur_length += 1;
            }
        }
    }

    return unpacked_array;
}

/**
 * Applies a specific byte modification to a resource.
 */
function patchResource(resource_map, resource_name, offset, before_byte, after_byte) {
    if (!resource_map.has(resource_name)) {
        const packed_resource_name = getPackedResourceName(resource_name);
        if (!packed_resource_name || !resource_map.has(packed_resource_name)) {
            logError("Could not find resource " + resource_name);
            return false;
        }
        const unpacked_resource = unpackResource(resource_map.get(packed_resource_name));
        if (unpacked_resource === null) {
            logError("Failed to unpack resource " + resource_name);
            return false;
        }
        resource_map.delete(packed_resource_name);
        resource_map.set(resource_name, unpacked_resource);
    }

    let resource_data = resource_map.get(resource_name);
    let original_byte = resource_data[offset];
    if (original_byte == before_byte) {
        resource_data[offset] = after_byte;
        console.log("Patch for " + resource_name + " offset " + offset.toString(16) + " applied successfully")
        return true;
    } else if (original_byte == after_byte) {
        console.log("Patch for " + resource_name + " offset " + offset.toString(16) + " is already applied")
        return true;
    } else {
        logError("Patch for " + resource_name + " offset " + offset.toString(16) + " could not be applied: byte is " + original_byte.toString(16) + ", but expected " + before_byte.toString(16));
        return false;
    }
}

/**
 * Applies all patches needed for WINTER.EXE
 */
function winter_patches(resource_map) {
    if (!patchResource(resource_map, "OVL1.COD", 0x0d21, 0x75, 0xeb)) { return false; } // always skip code wheel check
    if (!patchResource(resource_map, "OVL1.COD", 0x0d47, 0x74, 0xeb)) { return false; } // always pass debugger check
    if (!patchResource(resource_map, "OVL4.COD", 0x1f83, 0x74, 0xeb)) { return false; } // always pass downhill gravity change copy protection
    if (!patchResource(resource_map, "OVL4.COD", 0x42e9, 0x74, 0xeb)) { return false; } // always pass biathlon shot miss copy protection
    if (!patchResource(resource_map, "OVL5.COD", 0x2638, 0x74, 0xeb)) { return false; } // always pass speed skating outwards drift copy protection
    if (!patchResource(resource_map, "OVL6.COD", 0x2200, 0x76, 0xeb)) { return false; } // always pass ski jumping landing copy protection
    if (!patchResource(resource_map, "OVL7.COD", 0x1b0c, 0x74, 0xeb)) { return false; } // always pass luge forfeit copy protection
    if (!patchResource(resource_map, "OVL7.COD", 0x276f, 0x74, 0xeb)) { return false; } // always pass bobsled slow down copy protection
    return true;
}

/**
 * Applies all patches needed for SUMMER.EXE
 */
function summer_patches(resource_map) {
    if (!patchResource(resource_map,  "OVL1.COD", 0x1d5c, 0x55, 0xc3)) { return false;} // always skip signature check
    if (!patchResource(resource_map,  "OVL1.COD", 0x1e91, 0x74, 0xeb)) { return false;} // always pass debugger check
    if (!patchResource(resource_map,  "OVL1.COD", 0x1e69, 0x75, 0xeb)) { return false;} // always skip code wheel check
    if (!patchResource(resource_map, "OVL13.COD", 0x0eae, 0x74, 0xeb)) { return false;} // always pass cycling outwards drift copy protection
    if (!patchResource(resource_map, "OVL14.COD", 0x0ca8, 0x74, 0xeb)) { return false;} // always pass kayak left drift copy protection
    if (!patchResource(resource_map, "OVL15.COD", 0x13e0, 0x75, 0xeb)) { return false;} // always pass hurdles stumble copy protection
    if (!patchResource(resource_map, "OVL16.COD", 0x08ee, 0x74, 0xeb)) { return false;} // always pass pole vault release copy protection
    if (!patchResource(resource_map, "OVL17.COD", 0x0793, 0x74, 0xeb)) { return false;} // always pass high jump lift off copy protection
    if (!patchResource(resource_map, "OVL18.COD", 0x0ff9, 0x74, 0xeb)) { return false;} // always pass javelin throw angle copy protection
    return true;
}

/**
 * Extracts the resources from the given binary, applies the given patches, and re-assembles then into a new binary.
 * @param {ArrayBuffer} buffer
 * @returns {ArrayBuffer | null} The new binary, or null if an error occurred
 */
async function producePatchedBytes(buffer, patches_fn) {
    const data_view = new DataView(buffer);

    // extract the resources out of the binary
    if (data_view.getUint16(0, LITTLE_ENDIAN) !== 0x5a4d) {
        logError("MZ header not found. Make sure to select the correct file.");
        return null;
    }
    console.log("MZ header found.");

    const last_page_bytes = data_view.getUint16(2, LITTLE_ENDIAN);
    const page_count = data_view.getUint16(4, LITTLE_ENDIAN);
    let resource_data_start = (last_page_bytes == 0 ? page_count * PAGE_SIZE : (page_count - 1) * PAGE_SIZE + last_page_bytes);
    const header_size = data_view.getUint16(8, LITTLE_ENDIAN);

    let header_data = new Uint8Array(buffer, 0, header_size * PARAGRAPH_SIZE);
    let code_data = new Uint8Array(buffer, header_size * PARAGRAPH_SIZE, resource_data_start - header_size * PARAGRAPH_SIZE);

    if (data_view.getUint16(resource_data_start, LITTLE_ENDIAN) !== 0x424d) {
        logError("MB resource header not found. Make sure to select the correct file.");
        return null;
    }
    console.log("MB resource header found.");

    const resource_count = data_view.getUint16(resource_data_start + 2, LITTLE_ENDIAN);
    let resource_map = new Map();
    for (let i = 0; i < resource_count; i++) {
        const length = data_view.getUint32(resource_data_start + 4 + 22*i, LITTLE_ENDIAN);
        const start_index = data_view.getUint32(resource_data_start + 4 + 22*i + 4, LITTLE_ENDIAN);
        let file_name = "";
        for (let j = 8; j < 22; j++) {
            const char = data_view.getUint8(resource_data_start + 4 + 22*i + j);
            if (char == 0) {
                break;
            }
            file_name += String.fromCharCode(char - 0x60);
        }
        resource_map.set(file_name, new Uint8Array(buffer, start_index, length));
    }

    // modify resources
    if (!patches_fn(resource_map)) {
        return null;
    }
    
    // create new binary
    let output_length = header_data.byteLength + code_data.byteLength + 4 + 22 * resource_map.size;
    let resource_positions = new Map();
    for (let resource_name of resource_map.keys()) {
        output_length = 2 * Math.trunc((output_length + 1) / 2);
        resource_positions.set(resource_name, output_length);
        output_length += resource_map.get(resource_name).byteLength;
    }
    let output = new ArrayBuffer(output_length);
    let output_bytes = new Uint8Array(output);
    let output_data_view = new DataView(output);
    output_bytes.set(header_data, 0);
    output_bytes.set(code_data, header_data.byteLength);
    output_data_view.setUint16(header_data.byteLength + code_data.byteLength, 0x424d, LITTLE_ENDIAN);
    output_data_view.setUint16(header_data.byteLength + code_data.byteLength + 2, resource_map.size, LITTLE_ENDIAN);
    let current_offset = header_data.byteLength + code_data.byteLength + 4;
    for (let resource_name of resource_map.keys()) {
        output_data_view.setUint32(current_offset, resource_map.get(resource_name).byteLength, LITTLE_ENDIAN);
        output_data_view.setUint32(current_offset + 4, resource_positions.get(resource_name), LITTLE_ENDIAN);
        for (let i = 0; i < resource_name.length; i++) {
            output_data_view.setUint8(current_offset + 8 + i, resource_name.charCodeAt(i) + 0x60);
        }
        output_data_view.setUint8(current_offset + 8 + resource_name.length, 0);

        output_bytes.set(resource_map.get(resource_name), resource_positions.get(resource_name));

        current_offset += 22;
    }

    return output;
}

/**
 * Patches a given file, by reading the file, determining which modifications need to be applied, and providing a link to the updated file.
 * @param {File} file
 */
async function processInputFile(file) {
    const old_url = document.getElementById("download-btn").getAttribute("href");
    if (old_url) {
        URL.revokeObjectURL(old_url);
    }
    document.getElementById("download-btn").style.display = "none";
    initLog();

    console.log(`file name: ${file.name}`);
    let buffer = await file.arrayBuffer();
    let patched_buffer;
    if (file.name.toLowerCase() === "winter.exe") {
        patched_buffer = await producePatchedBytes(buffer, winter_patches);
    } else if (file.name.toLowerCase() === "summer.exe") {
        patched_buffer = await producePatchedBytes(buffer, summer_patches);
    } else {
        logError("Unrecognized file name \"" + file.name + "\". Make sure you selected the correct file.")
    }
    if (!patched_buffer) {
        return;
    }

    logSuccess(file.name + " successfully patched.");
    const patched_binary_blob = new Blob([patched_buffer], { type: "application/octet-stream" });
    const patched_binary_url = URL.createObjectURL(patched_binary_blob);
    document.getElementById("download-btn").setAttribute("href", patched_binary_url);
    document.getElementById("download-btn").setAttribute("download", file.name);
    document.getElementById("download-btn").style.display = "block";
}

/**
 * @param {Event} ev
 */
async function handleFileInputChange(ev) {
    if (ev.target.files) {
        for (const file of ev.target.files) {
            await processInputFile(file);
            return;
        }
    }
}

/**
 * @param {DragEvent} ev
 */
async function dropHandler(ev) {
    // Prevent default behavior (Prevent file from being opened)
    ev.preventDefault();

    if (ev.dataTransfer.items) {
        for (const item of ev.dataTransfer.items) {
            // If dropped items aren't files, reject them
            if (item.kind === "file") {
                const file = item.getAsFile();
                await processInputFile(file);
                return;
            }
        }
    }
    if (ev.dataTransfer.files) {
        for (const file in ev.dataTransfer.files) {
            await processInputFile(file);
            return;
        }
    }
}

/**
 * @param {DragEvent} ev
 */
function dragOverHandler(ev) {
    // Prevent default behavior (Prevent file from being opened)
    ev.preventDefault();
}
