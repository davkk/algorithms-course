let bisect (min: float) (max: float) (f: float -> float) =
    let rec loop (a: float) (b: float) =
        let midpoint = a + (b - a) / 2.0

        let result = f midpoint
        let error = 1e-10

        if result < error && result > -error then
            midpoint
        elif result > 0.0 then
            loop a midpoint
        else
            loop midpoint b

    loop min max

(fun x ->
    (x + 2.0)
    * (x + 1.0)
    * x
    * (x - 1.) ** 3.
    * (x - 2.)
)
|> bisect -3.0 2.5
