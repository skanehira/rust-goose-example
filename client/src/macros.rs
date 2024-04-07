#[macro_export]
macro_rules! create_scenario {
    ($attack: expr, $name: expr, $scenarios: expr, $param_ty: ty, $func_creator: ident) => {{
        // パラメータファイルから$nameと一致するシナリオを探す
        let scenario = $scenarios
            .iter()
            .find(|s| s.name == $name)
            .expect(&format!(
                "not found scenario parameters of {}",
                $name
            ));

        // yamlに記述したパラメータを元にシナリオ関数を作成
        let param = ScenarioParam::<$param_ty>::try_from(scenario.clone()).expect("failed to create scenario param");
        $attack = $attack.register_scenario($func_creator($name, param));
    }};
}
