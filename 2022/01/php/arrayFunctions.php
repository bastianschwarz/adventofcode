<?php declare(strict_types=1);

$lines = file(__DIR__ . '/list', FILE_IGNORE_NEW_LINES);

$currentElf = 0;
$elves = array_reduce($lines, static function (array $calories, string $line) use (&$currentElf): array {
    if ($line === '') {
        $currentElf++;
    } else {
        array_key_exists($currentElf, $calories) ? $calories[$currentElf] += (int)$line : $calories[$currentElf] = (int)$line;
    }
    return $calories;
}, []);
$highestCalories = max($elves);
$elfWithHighestCalories = array_search($highestCalories, $elves, true) + 1;

echo "The elf with the most calories is number $elfWithHighestCalories with $highestCalories calories.";
