let binarySearch (needle: float) (arr: float array) =
    let sorted = arr |> Array.sort

    let rec loop (min: int) (max: int) : int =
        let midpoint = min + (max - min) / 2

        System.Console.WriteLine midpoint

        if sorted.[midpoint] = needle then
            midpoint
        elif sorted.[midpoint] > needle then
            loop min midpoint
        else
            loop (midpoint + 1) max

    loop 0 arr.Length

[| 0. .. 12390813412. |] |> binarySearch 42.
