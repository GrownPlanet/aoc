type rope = { pos : int * int; tail : rope option }

module CoordSet = Set.Make (struct
  type t = int * int

  let compare = compare
end)

let get_offset hx hy tx ty = (abs (tx - hx), abs (ty - hy))

let move_head rope direction =
  let hx, hy = rope.pos in
  match direction with
  | "U" -> { rope with pos = (hx, hy - 1) }
  | "D" -> { rope with pos = (hx, hy + 1) }
  | "L" -> { rope with pos = (hx - 1, hy) }
  | "R" -> { rope with pos = (hx + 1, hy) }
  | _ -> invalid_arg "invalid direction"

let rec move_tail head =
  match head.tail with
  | Some tail ->
      let hx, hy = head.pos in
      let tx, ty = tail.pos in
      let new_tail_pos =
        match get_offset hx hy tx ty with
        | 0, 2 | 1, 2 -> (hx, (ty + hy) / 2)
        | 2, 0 | 2, 1 -> ((hx + tx) / 2, hy)
        | 2, 2 -> ((hx + tx) / 2, (ty + hy) / 2)
        | _ -> (tx, ty)
      in
      let new_tail = { tail with pos = new_tail_pos } |> move_tail in
      { head with tail = Some new_tail }
  | None ->
      head

let rec update_visited visited rope =
  match rope.tail with
  | Some tail -> update_visited visited tail
  | None -> CoordSet.add rope.pos visited

let rec drag visited rope direction count =
  if count <= 0 then (rope, visited)
  else
    let new_rope = move_head rope direction |> move_tail in
    let new_visited =
      match new_rope.tail with
      | Some tail -> update_visited visited tail
      | None -> visited
    in
    drag new_visited new_rope direction (count - 1)

let parse_line line =
  match String.split_on_char ' ' line with
  | [ dir; count ] -> (dir, int_of_string count)
  | _ -> invalid_arg "unparsable line"

let rec exec_lines rope visited lines =
  match lines with
  | [] -> visited
  | line :: r ->
      let dir, count = parse_line line in
      let new_rope, new_visited = drag visited rope dir count in
      exec_lines new_rope new_visited r

let read_input filename =
  In_channel.with_open_text filename In_channel.input_lines

let rec create_rope length =
  if length <= 0 then None
  else Some { pos = (0, 0); tail = create_rope (length - 1) }

let () =
  let input = read_input "input.txt" in
  let rope = Option.get (create_rope 10) in
  let visited = exec_lines rope CoordSet.empty input |> CoordSet.to_list in
  List.length visited |> Printf.printf "%d\n"
