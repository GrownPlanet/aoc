let read_input filename =
  In_channel.with_open_text filename In_channel.input_lines

let parse line = Scanf.sscanf line "%d,%d" (fun x y -> (x, y))
let get_area (x1, y1) (x2, y2) = (abs (x2 - x1) + 1) * (abs (y2 - y1) + 1)

let rec get_areas_with p list =
  match list with p2 :: r -> get_area p p2 :: get_areas_with p r | [] -> []

let rec get_areas list =
  match list with p :: r -> get_areas_with p r @ get_areas r | [] -> []

let () =
  read_input "input.txt" |> List.map parse |> get_areas |> List.fold_left max 0
  |> Printf.printf "%d\n"
