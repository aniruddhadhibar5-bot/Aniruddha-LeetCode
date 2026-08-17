def gcd(a, b):
    while b:
        a, b = b, a % b
    return a

class Solution:
    def maxPoints(self, points):
        if len(points) <= 2:
            return len(points)

        max_points = 0
        for i in range(len(points)):
            slopes = {}
            duplicates = 1
            for j in range(i + 1, len(points)):
                x1, y1 = points[i]
                x2, y2 = points[j]
                dx, dy = x2 - x1, y2 - y1

                if dx == 0 and dy == 0:
                    # Same point
                    duplicates += 1
                    continue

                g = gcd(dx, dy)
                dx //= g
                dy //= g

                # Normalize slope direction
                if dx < 0:
                    dx, dy = -dx, -dy
                elif dx == 0:
                    dy = 1
                elif dy == 0:
                    dx = 1

                slopes[(dx, dy)] = slopes.get((dx, dy), 0) + 1

            max_points = max(max_points, duplicates + (max(slopes.values()) if slopes else 0))
        return max_points
