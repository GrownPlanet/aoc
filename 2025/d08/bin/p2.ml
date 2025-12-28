let read_input filename =
  In_channel.with_open_text filename In_channel.input_lines

let parse input =
  List.map
    (fun pos -> Scanf.sscanf pos "%d,%d,%d" (fun x y z -> (x, y, z)))
    input

let square n = n * n
let compare_points (x1, y1, z1) (x2, y2, z2) = x1 = x2 && z1 = z2 && y1 = y2

let distance (x1, y1, z1) (x2, y2, z2) =
  square (x2 - x1) + square (y2 - y1) + square (z2 - z1)

let find_distances_to_point point points =
  List.map (fun p -> (distance p point, point, p)) points

let find_distances points =
  let rec aux acc points =
    match points with
    | p :: rest -> aux (find_distances_to_point p rest @ acc) rest
    | [] -> acc
  in
  let distances = aux [] points |> Array.of_list in
  Array.fast_sort (fun (a, _, _) (b, _, _) -> compare a b) distances;
  Array.to_list distances

let rec find_root cons p =
  let r = Hashtbl.find cons p in
  if compare_points r p then r else find_root cons r

let rec solve distances cons graph_count =
  match distances with
  | (_, p1, p2) :: rest ->
      let p1r = find_root cons p1 in
      let p2r = find_root cons p2 in
      Hashtbl.replace cons p2r p1r;
      let new_graph_count =
        if compare_points p1r p2r then graph_count else graph_count - 1
      in
      if new_graph_count = 1 then
        let x1, _, _ = p1 in
        let x2, _, _ = p2 in
        x1 * x2
      else solve rest cons new_graph_count
  | [] -> invalid_arg "no more distances"

let () =
  let input = read_input "input.txt" |> parse in
  let distances = find_distances input in
  let input_len = List.length input in
  let cons = Hashtbl.create input_len in
  List.iter (fun p -> Hashtbl.add cons p p) input;
  solve distances cons input_len |> Printf.printf "%d\n"
