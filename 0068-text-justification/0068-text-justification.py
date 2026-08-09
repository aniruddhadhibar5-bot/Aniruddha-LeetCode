class Solution:
    def fullJustify(self, words, maxWidth):
        res = []
        i = 0
        n = len(words)

        while i < n:
            # Step 1: Greedy packing
            line_len = len(words[i])
            j = i + 1
            while j < n and line_len + 1 + len(words[j]) <= maxWidth:
                line_len += 1 + len(words[j])
                j += 1

            # Step 2: Build line
            line_words = words[i:j]
            spaces_needed = maxWidth - sum(len(w) for w in line_words)
            gaps = len(line_words) - 1

            # Last line OR single word → left-justified
            if j == n or gaps == 0:
                line = " ".join(line_words)
                line += " " * (maxWidth - len(line))
            else:
                # Distribute spaces evenly
                space, extra = divmod(spaces_needed, gaps)
                line = ""
                for k in range(gaps):
                    line += line_words[k]
                    line += " " * (space + (1 if k < extra else 0))
                line += line_words[-1]

            res.append(line)
            i = j

        return res
