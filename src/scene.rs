use crate::{
    aabb::*, camera::*, hit::*, material::*, ray::*, sphere::*, texture::*, utility::*, vec3::*,
};
pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let checker = Arc::new(CheckerTexture::new_by_color(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        -1000.0,
        Arc::new(Lambertian::new(checker)),
    )));

    // let ground_material = Arc::new(Lambertian::new_by_color(
    //     Vec3::new(0.5,0.5,0.5),
    // ));
    // world.objects.push(Box::new(Sphere::new(Vec3::new(0.0,-1000.0,0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Vec3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );
            if (center - Vec3::new(4.0, 0.4, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::elemul(Vec3::random(), Vec3::random());
                    sphere_material = Arc::new(Lambertian::new_by_color(albedo));
                    let center2 = center + Vec3::new(0.0, random_in_range_f64(0.0, 0.5), 0.0);
                    world.objects.push(Box::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else {
                    if choose_mat < 0.95 {
                        // metal
                        let albedo = Vec3::random_in_range(0.5, 1.0);
                        let fuzz = random_in_range_f64(0.0, 0.5);
                        sphere_material = Arc::new(Metal {
                            albedo: albedo,
                            fuzz: fuzz,
                        });
                        world
                            .objects
                            .push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    } else {
                        // glass
                        sphere_material = Arc::new(Dielectric { ref_idx: 1.5 });
                        world
                            .objects
                            .push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                }
            }
        }
    }
    let material1 = Arc::new(Dielectric { ref_idx: 1.5 });
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new_by_color(Vec3::new(0.4, 0.2, 0.1)));
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    return world;
}

pub fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let checker = Arc::new(CheckerTexture::new_by_color(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    objects.objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker.clone())),
    )));
    objects.objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker.clone())),
    )));
    return objects;
}

pub fn simple_light() -> HittableList{
    let mut objects: HittableList = HittableList::new();

    let pertext = Arc::new(SolidColor::new(Vec3::new(0.8,0.8,0.8)));
    objects.objects.push(Box::new(Sphere::new(Vec3::new(0.0,-1000.0,0.0),1000.0,Arc::new(Lambertian::new(pertext)))));

    let difflight = Arc::new(DiffuseLight::new_by_color(Vec3::new(4.0,4.0,4.0)));
    let lamber = Arc::new(Lambertian::new_by_color(Vec3::new(0.8,0.8,0.8)));
    objects.objects.push(Box::new(Sphere::new(Vec3::new(0.0,2.0,0.0),1.0,difflight)));
    objects.objects.push(Box::new(Sphere::new(Vec3::new(0.0,2.0,3.0),1.0,lamber)));
    
    return objects;
}