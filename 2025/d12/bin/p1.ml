(*
See day 2 for the "proper" solution
*)
(*
let read_input filename = In_channel.(with_open_text filename input_lines)

let rec split_double_nl l_acc ll_acc lines =
  match lines with
  | "" :: r -> split_double_nl [] (List.rev l_acc :: ll_acc) r
  | l :: r -> split_double_nl (l :: l_acc) ll_acc r
  | [] -> List.rev (l_acc :: ll_acc)

let parse_shape shape = Array.of_list (List.tl shape)

let parse_region region =
  Scanf.sscanf region "%dx%d: %s"
    (fun width height values ->
      let indices =
        String.split_on_char ' ' values |> List.filter_map int_of_string_opt
      in
      (width, height, indices))

let parse_regions regions = List.map parse_region regions

let rec parse_blocks shapes blocks =
  match blocks with
  | [] -> invalid_arg "invalid input"
  | [ e ] -> (List.rev shapes, parse_regions e)
  | shape :: r -> parse_blocks (parse_shape shape :: shapes) r

let parse lines = split_double_nl [] [] lines |> parse_blocks []

module ShapeSet = Set.Make (struct
  type t = string array

  let compare = compare
end)

let transform f shape =
  let len = Array.length shape in
  Array.init len (fun i ->
      String.init len (fun j ->
          let ni, nj = f len i j in
          String.get shape.(ni) nj))

let rotate_shape = transform (fun l i j -> (l - 1 - j, i))
let mirror_v = transform (fun l i j -> (i, l - 1 - j))
let mirror_h = transform (fun l i j -> (l - 1 - i, j))

let get_rotations shape =
  let r1 = rotate_shape shape in
  let r2 = rotate_shape r1 in
  let r3 = rotate_shape r2 in
  [ shape; r1; r2; r3 ]

let shape_permutations shape =
  let rotations = get_rotations shape in
  let mirv = mirror_v shape |> get_rotations in
  let mirh = mirror_h shape |> get_rotations in
  let permutations =
    ShapeSet.(
      empty
      |> union (of_list rotations)
      |> union (of_list mirv)
      |> union (of_list mirh))
  in
  permutations |> ShapeSet.to_seq |> Array.of_seq

let fits ((w, h), quant) shapes =
  let _ = w in
  let _ = h in
  let _ = quant in
  let _ = shapes in
  true

let () =
  let shapes, regions = read_input "test_input.txt" |> parse in
  let shapes = List.map shape_permutations shapes in
  List.fold_left
    (fun acc region -> if fits region shapes then acc + 1 else acc)
    0 regions
  |> Printf.printf "%d\n"
*)
