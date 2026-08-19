public class Solution {
    public bool ReachingPoints(int sx, int sy, int tx, int ty) {
        // Work backward while target values are greater than start values
        while (tx > sx && ty > sy) {
            if (tx > ty) {
                tx %= ty;
            } else {
                ty %= tx;
            }
        }
        
        // If tx matches sx, ty must be reduced to sy by subtracting tx repeatedly
        if (tx == sx) {
            return ty >= sy && (ty - sy) % tx == 0;
        }
        
        // If ty matches sy, tx must be reduced to sx by subtracting ty repeatedly
        if (ty == sy) {
            return tx >= sx && (tx - sx) % ty == 0;
        }
        
        // If neither matched, it's impossible
        return false;
    }
}
