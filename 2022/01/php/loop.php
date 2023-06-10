<?php declare(strict_types=1);

$file = fopen(__DIR__ . '/list', 'rb');

$elfWithMostCalories = (object)['number' => 0, 'calories' => 0];
$caloriesForCurrentElf = 0;
$currentElf = 1;
while (($line = fgets($file)) !== false) {
    if (trim($line) === '') {
        if ($caloriesForCurrentElf > $elfWithMostCalories->calories) {
            $elfWithMostCalories->calories = $caloriesForCurrentElf;
            $elfWithMostCalories->number = $currentElf;
        }
        $caloriesForCurrentElf = 0;
        $currentElf++;
    } else {
        $caloriesForCurrentElf += (int)trim($line);
    }
}

echo "The elf with the most calories is number {$elfWithMostCalories->number} with {$elfWithMostCalories->calories} calories.";
