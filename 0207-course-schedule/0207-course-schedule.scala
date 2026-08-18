import scala.collection.mutable

object Solution {
    def canFinish(numCourses: Int, prerequisites: Array[Array[Int]]): Boolean = {
        val adj = Array.fill(numCourses)(mutable.ListBuffer[Int]())
        val inDegree = new Array[Int](numCourses)

        // 1. Build adjacency list and calculate in-degrees
        for (edge <- prerequisites) {
            val dest = edge(0)
            val src = edge(1)
            adj(src) += dest
            inDegree(dest) += 1
        }

        // 2. Add all courses with 0 prerequisites to the queue
        val queue = mutable.Queue[Int]()
        for (i <- 0 until numCourses) {
            if (inDegree(i) == 0) {
                queue.enqueue(i)
            }
        }

        // 3. Process the queue
        var processedCount = 0
        while (queue.nonEmpty) {
            val curr = queue.dequeue()
            processedCount += 1

            for (neighbor <- adj(curr)) {
                inDegree(neighbor) -= 1
                if (inDegree(neighbor) == 0) {
                    queue.enqueue(neighbor)
                }
            }
        }

        // 4. If processed count matches total courses, no cycle exists
        processedCount == numCourses
    }
}
