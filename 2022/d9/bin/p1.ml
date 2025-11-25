type rope = { head : int * int; tail : int * int }

module CoordSet = Set.Make (struct
  type t = int * int

  let compare = compare
end)

let get_offset rope =
  let hx, hy = rope.head in
  let tx, ty = rope.tail in
  (abs (tx - hx), abs (ty - hy))

let move_head rope direction =
  let hx, hy = rope.head in
  match direction with
  | "U" -> { rope with head = (hx, hy - 1) }
  | "D" -> { rope with head = (hx, hy + 1) }
  | "L" -> { rope with head = (hx - 1, hy) }
  | "R" -> { rope with head = (hx + 1, hy) }
  | _ -> invalid_arg "invalid direction"

let move_tail rope =
  let hx, hy = rope.head in
  let tx, ty = rope.tail in
  match get_offset rope with
  | 0, 2 | 1, 2 -> { rope with tail = (hx, (ty + hy) / 2) }
  | 2, 0 | 2, 1 -> { rope with tail = ((hx + tx) / 2, hy) }
  | _ -> rope

let rec drag visited rope direction count =
  if count <= 0 then (rope, visited)
  else
    let new_rope = move_head rope direction |> move_tail in
    let new_visited = CoordSet.add new_rope.tail visited in
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

let () =
  let input = read_input "input.txt" in
  let rope = { head = (0, 0); tail = (0, 0) } in
  let visited = exec_lines rope CoordSet.empty input in
  CoordSet.to_list visited |> List.length |> Printf.printf "%d\n"
