type Edge = { To: int; Weight: int }
type AdjacencyList = Edge list list

let list1: AdjacencyList =
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

let dfs (source: int) (needle: int) (graph: AdjacencyList) =
    let rec walk (seen: int Set) (stack: int list) =
        [
            match stack with
            | [] -> ()
            | curr :: remaining ->
                if curr = needle then
                    yield curr

                elif (seen |> Set.contains curr) then
                    yield! walk seen remaining

                else
                    let adjacent =
                        graph[curr]
                        |> List.map (fun edge -> edge.To)

                    yield! walk (seen |> Set.add curr) (adjacent @ remaining)
        ]

    walk (Set.empty) [ source ]

list1 |> dfs 0 6
