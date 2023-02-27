
/*A library that returns back a random food name */

use rand::Rng;

//create an const array of the top 10 best food around the world
pub const FOOD: [&str; 10] = [
    "Huevos rancheros with refried beans and salsa.",
    "Veggie burger on a whole wheat bun with sweet potato fries",
    "Baked chicken with mashed potatoes and green beans",
    "Breakfast burrito with scrambled eggs, black beans, avocado, and salsa",
    "Grilled cheeseburger with sweet potato wedges",
    "Spaghetti and meatballs with a side of garlic bread and Caesar salad",
    "Overnight oats with chia seeds, almond milk, and mixed berries",
    "Tuna salad sandwich on whole grain bread with carrot sticks",
    "Baked salmon with brown rice and steamed broccoli",
    "Greek yogurt with sliced banana, granola, and honey",
];

//create a function that returns a random food in the list above
pub fn random_food() -> &'static str {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..FOOD.len());
    FOOD[random_index]
}
