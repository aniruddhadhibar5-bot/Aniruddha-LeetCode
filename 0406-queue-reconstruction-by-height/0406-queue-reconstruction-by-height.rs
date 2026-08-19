impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 1. लोगों को सॉर्ट करें:
        // - कद (height) के हिसाब से घटते क्रम (descending) में
        // - समान कद होने पर k के हिसाब से बढ़ते क्रम (ascending) में
        people.sort_by(|a, b| {
            if a[0] != b[0] {
                b[0].cmp(&a[0]) // घटता क्रम
            } else {
                a[1].cmp(&b[1]) // बढ़ता क्रम
            }
        });

        let mut result: Vec<Vec<i32>> = Vec::with_capacity(people.len());

        // 2. हर व्यक्ति को उसके k मान वाले इंडेक्स पर डालें
        for person in people {
            let idx = person[1] as usize;
            result.insert(idx, person);
        }

        result
    }
}
