let read_input filename =
  In_channel.with_open_text filename In_channel.input_lines

let rec append a b =
  match (a, b) with
  | v :: ra, l :: rb -> (v :: l) :: append ra rb
  | a, [] -> List.map (fun e -> [ e ]) a
  | _ -> invalid_arg "wrong"

let split_input inp =
  String.split_on_char ' ' inp |> List.filter (fun s -> s <> "")

let rec parse_nums nums =
  match nums with
  | num_arr :: r ->
      let nums = split_input num_arr |> List.map int_of_string in
      append nums (parse_nums r)
  | _ -> []

let opp_to_func opp =
  match opp with
  | "*" -> ( * )
  | "+" -> ( + )
  | _ -> invalid_arg "invalid opperation"

let begin_opp opp =
  match opp with "*" -> 1 | "+" -> 0 | _ -> invalid_arg "invalid opperation"

let rec solve opps nums =
  match (opps, nums) with
  | opp :: ropps, num :: rnums ->
      List.fold_left (opp_to_func opp) (begin_opp opp) num + solve ropps rnums
  | _ -> 0

let () =
  match read_input "input.txt" |> List.rev with
  | opps :: nums ->
      nums |> parse_nums |> solve (split_input opps) |> Printf.printf "%d\n"
  | [] -> invalid_arg "empty input"
