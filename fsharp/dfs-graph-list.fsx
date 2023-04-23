open System.Collections.Generic

type Edge = { To: int; Weight: int }
type AdjacencyList = Edge list list

let dfs (source: int) (needle: int) (path: int Stack) (graph: AdjacencyList) =
    let rec walk (curr: int) (seen: int Set) =
        printfn "path: %A" (path |> Seq.toList)
        printfn "seen: %A" seen

        if curr = needle then
            printfn "-- found %d" curr
            path.Push(curr)
            true
        elif seen |> Set.contains curr then
            printfn "-- already seen %d" curr
            false
        else
            printfn "-- not seen %d" curr
            path.Push(curr)

            let seen = seen.Add(curr)
            let edges = graph[curr]

            let result =
                edges
                |> List.tryFind (fun edge -> walk edge.To seen)

            match result with
            | Some _ -> true
            | None ->
                let _ = path.Pop()
                false

    walk source Set.empty


// # example graphs are taken from ThePrimeagen's tests

// ## list1:
//      (1) --- (4) ---- (5)
//    /  |       |       /|
// (0)   | ------|------- |
//    \  |/      |        |
//      (2) --- (3) ---- (6)
let list1: AdjacencyList =
    [
        [ { To = 1; Weight = 3 }; { To = 2; Weight = 1 } ]
        [
            { To = 0; Weight = 3 }
            { To = 2; Weight = 4 }
            { To = 4; Weight = 1 }
        ]
        [
            { To = 1; Weight = 4 }
            { To = 3; Weight = 7 }
            { To = 0; Weight = 1 }
        ]
        [
            { To = 2; Weight = 7 }
            { To = 4; Weight = 5 }
            { To = 6; Weight = 1 }
        ]
        [
            { To = 1; Weight = 1 }
            { To = 3; Weight = 5 }
            { To = 5; Weight = 2 }
        ]
        [
            { To = 6; Weight = 1 }
            { To = 4; Weight = 2 }
            { To = 2; Weight = 18 }
        ]
        [ { To = 3; Weight = 1 }; { To = 5; Weight = 1 } ]
    ]

// ## list2:
//     >(1)<--->(4) ---->(5)
//    /          |       /|
// (0)     ------|------- |
//    \   v      v        v
//     >(2) --> (3) <----(6)
let list2: AdjacencyList =
    [
        [ { To = 1; Weight = 3 }; { To = 2; Weight = 1 } ]
        [ { To = 4; Weight = 1 } ]
        [ { To = 3; Weight = 7 } ]
        []
        [
            { To = 1; Weight = 1 }
            { To = 3; Weight = 5 }
            { To = 5; Weight = 2 }
        ]
        [ { To = 2; Weight = 18 }; { To = 6; Weight = 1 } ]
        [ { To = 3; Weight = 1 } ]
    ]

let path1 = Stack()
let _ = list1 |> dfs 0 6 path1
path1 |> Seq.rev |> Seq.toList // = [0; 1; 2; 3; 4; 5; 6]

let path2 = Stack()
let _ = list2 |> dfs 0 6 path2
path2 |> Seq.rev |> Seq.toList // = [0; 1; 4; 5; 6]
