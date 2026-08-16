/**
 * @param {Object|Array} obj
 * @return {boolean}
 * @time O(1) - Using an iterator provides instant true/false feedback
 * @space O(1) - Constant space usage
 */
var isEmpty = function(obj) {
    for (const key in obj) {
        return false;
    }
    return true;
};
