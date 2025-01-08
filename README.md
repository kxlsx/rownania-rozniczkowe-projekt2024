# rownania-rozniczkowe-projekt2024

Repozytorium zawiera projekt na laboratoria
z Równań Różniczkowych i Różnicowych na
WI AGH 2024/2025.

## Opis zadania

Celem projektu bylo znalezienie rozwiązania
dla równania różniczkowego opisującego
potencjał grawitacyjny w jednym wymiarze:

$$
    {d^2 \Phi \over dx^2} = 4\pi G \rho(x)
    \\ ~ \\
    \Phi(0) = 5, \quad \Phi(3) = 4, \quad \Omega = (0; 3)
    \\ ~ \\
    \rho(x)=\left\{
    \begin{array}{ll}
        0 & x \in [0; 1] \\
        1 & x \in (1; 2] \\
        0 & x \in (2; 3] \\ 
    \end{array} 
    \right.
$$

za pomocą metody elementów skończonych.

## Rozwiązanie

Wykonane przeze mnie obliczenia znajdują się w pliku [calc.pdf](https://github.com/kxlsx/rownania-rozniczkowe-projekt2024/blob/main/calc.pdf).
Implementacja algorytmu aproksymującego wartość $\Phi(x)$ została
napisana w języku `Rust`. Przykładowy wygenerowany wykres znajduje
się w pliku [res/plot.png](https://github.com/kxlsx/rownania-rozniczkowe-projekt2024/blob/main/res/plot.png)

## Korzystanie z programu

Program należy skompilować za pomocą narzędzia `cargo` dostępnego
razem z kompilatorem języka `Rust` pod adresem: https://www.rust-lang.org/tools/install

Po kompilacji, program można wywołać w terminalu.
Domyślnym zachowaniem jest wypisanie na standardowe wyjście trzech macierzy przedstawiających układ równań liniowych i jego rozwiązanie
oraz zapisanie wykresu obliczonego przybliżenia funkcji $\Phi(x)$
do pliku `./a.png`. Ilość elementów ustawiona jest początkowo jako 20.

Zachowanie programu można modyfikować za pomocą flag.
W celu sprawdzenia dostępnych opcji, należy wykonać
program z flagą `--help`.
