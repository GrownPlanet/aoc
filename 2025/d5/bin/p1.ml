let read_input filename =
  In_channel.with_open_text filename In_channel.input_lines

let rec split_input acc lines =
  match lines with
  | line :: rest ->
      if line = "" then (acc, rest) else split_input (line :: acc) rest
  | _ -> invalid_arg "no blank line"

let parse_range range = Scanf.sscanf range "%d-%d" (fun f t -> (f, t))
let is_in_range v (f, t) = f <= v && v <= t

let () =
  let rranges, ringredients = read_input "input.txt" |> split_input [] in
  let ranges = List.map parse_range rranges in
  let ingredients = List.map int_of_string ringredients in
  List.map
    (fun i ->
      if
        List.map (fun range -> is_in_range i range) ranges
        |> List.fold_left ( || ) false
      then 1
      else 0)
    ingredients
  |> List.fold_left ( + ) 0 |> Printf.printf "%d\n"
