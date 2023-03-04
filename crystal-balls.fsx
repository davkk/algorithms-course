(*
    I am in a 100-story building. I have with me two crystal balls.

    I know that if I throw the ball out of the window, it will not break if the floor number is less than X,
    and it will always break if the floor number is equal to or greater than X.

    Assuming that I have only two balls, find X.
*)

let findX (arr: bool array) =
    let jump =
        arr.Length |> float |> sqrt |> floor |> int

    let rec loop (curr) =
        if curr > arr.Length || arr.[curr] then
            let start = curr - jump
            start + (arr.[start..] |> Array.findIndex id)
        else
            loop (curr + jump)

    loop jump

[| for _ in 0 .. 612314 -> false
   for _ in 0 .. 21 -> true |]
|> findX
