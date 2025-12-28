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

let rec find_root connections p =
  let r = Hashtbl.find connections p in
  if compare_points r p then r else find_root connections r

let rec solve distances connections conc =
  if conc < 1 then ()
  else
    match distances with
    | (_, p1, p2) :: rest ->
        let p1r = find_root connections p1 in
        let p2r = find_root connections p2 in
        Hashtbl.replace connections p2r p1r;
        solve rest connections (conc - 1)
    | [] -> invalid_arg "no more distances"

let get_sizes connections =
  let sizes = Hashtbl.create 100 in
  Hashtbl.iter
    (fun point _ ->
      let r = find_root connections point in
      match Hashtbl.find_opt sizes r with
      | None -> Hashtbl.add sizes r 1
      | Some n -> Hashtbl.replace sizes r (n + 1))
    connections;
  sizes

let () =
  let input = read_input "input.txt" |> parse in
  let distances = find_distances input in
  let connections =
    input |> List.to_seq |> Seq.map (fun x -> (x, x)) |> Hashtbl.of_seq
  in
  solve distances connections 1000;
  get_sizes connections |> Hashtbl.to_seq |> List.of_seq
  |> List.map (fun (_, x) -> x)
  |> List.sort (fun a b -> b - a)
  |> List.take 3 |> List.fold_left ( * ) 1 |> Printf.printf "%d\n"
