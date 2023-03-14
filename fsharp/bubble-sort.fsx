let bubbleSort (arr: int array) =
    let rec loop (arr: int array) =
        ()

    loop arr

let rng = System.Random()
let numbers = [| for _ in 1 .. 100 -> rng.Next() |]

System.Console.WriteLine numbers
System.Console.WriteLine (numbers |> bubbleSort)

