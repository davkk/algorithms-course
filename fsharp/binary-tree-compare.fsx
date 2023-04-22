type BinaryNode =
    {
        Value: int
        Left: BinaryNode option
        Right: BinaryNode option
    }

let rec compareBinaryTrees
    (a: BinaryNode option)
    (b: BinaryNode option)
    : bool =
    match a, b with
    | None, None -> true
    | Some _, None
    | None, Some _ -> false
    | Some a, Some b when a.Value <> b.Value -> false
    | Some a, Some b ->
        compareBinaryTrees a.Left b.Left
        && compareBinaryTrees a.Right b.Right

let tree1 =
    Some { Value = 1; Left = None; Right = None }

let tree2 =
    Some
        {
            Value = 1
            Left = None
            Right = Some { Value = 2; Left = None; Right = None }
        }

let tree3 =
    Some { Value = 1; Left = None; Right = None }

printfn $"tree1 = tree2? {compareBinaryTrees tree1 tree2}"
printfn $"tree1 = tree3? {compareBinaryTrees tree1 tree3}"
